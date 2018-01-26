use clap::{App, Arg, AppSettings};
#[cfg(unix)]
use base64;
#[cfg(unix)]
use libc;

use ::{Result, Shell, Arguments};

#[cfg(unix)]
use std::mem;
use std::result;
use std::process::Command;


pub fn echo(sh: &mut Shell, args: Arguments) -> Result<()> {
    let msg = match args.len() {
        0 | 1 => String::new(),
        _ => {
            let mut msg = args.into_iter().skip(1)
                .fold(String::new(), |a, b| {
                    a + " " + &b
                });
            msg.remove(0);
            msg
        },
    };

    shprintln!(sh, "{}", msg);

    Ok(())
}


#[cfg(unix)]
pub fn jit(sh: &mut Shell, args: Arguments) -> Result<()> {
    let matches = App::new("jit")
        .setting(AppSettings::DisableVersion)
        .arg(Arg::with_name("hex")
            .help("shellcode is hex encoded")
            .short("x")
        )
        .arg(Arg::with_name("shellcode")
            .help("base64 encoded shellcode")
            .required(true)
        )
        .get_matches_from_safe(args)?;

    let hex = matches.occurrences_of("hex") > 0;

    let shellcode = matches.value_of("shellcode").unwrap();
    let mut shellcode: Vec<u8> = {
        if hex {
            unhexify(shellcode).unwrap()
        } else {
            base64::decode(shellcode).unwrap()
        }
    };

    const PAGE_SIZE: usize = 4096;

    let num_pages = 1;
    let size = num_pages * PAGE_SIZE;
    let mut page: *mut libc::c_void = unsafe { mem::uninitialized() };

    unsafe { libc::posix_memalign(&mut page, PAGE_SIZE, size) };
    unsafe { libc::mprotect(page, size, libc::PROT_READ | libc::PROT_WRITE) };
    unsafe { libc::memcpy(page, shellcode.as_mut_ptr() as *mut libc::c_void, shellcode.len()) };
    unsafe { libc::mprotect(page, size, libc::PROT_READ | libc::PROT_EXEC) };

    shprintln!(sh, "shellcode: {:?} ({} bytes) \"{}\"", shellcode.as_ptr(), shellcode.len(), shellcode.iter()
        .fold(String::new(), |a, b| {
            a + &format!("\\x{:02X}", b)
        }));

    let shellcode = unsafe { mem::transmute::<*mut libc::c_void, extern fn() -> u32>(page) };

    shellcode();

    unsafe { libc::free(page) };

    Ok(())
}


// TODO: i/o isn't redirected
pub fn exec(_sh: &mut Shell, mut args: Arguments) -> Result<()> {
    if args.len() < 2 {
        // triggers an usage errror
        let _ = App::new("exec")
            .setting(AppSettings::DisableVersion)
            .arg(Arg::with_name("prog")
                .required(true)
            )
            .arg(Arg::with_name("args")
                .multiple(true)
            )
            .get_matches_from_safe(args)?;
    } else {
        let _ = args.remove(0);

        let prog = args.remove(0);

        Command::new(prog)
            .args(args)
            .spawn()?
            .wait()?;
    }

    Ok(())
}


fn unhexify(input: &str) -> result::Result<Vec<u8>, ()> {
    // the escape sequence parser translates "\xFF" to "xFF",
    // so work with that for now
    let bytes: Vec<char> = input.chars()
        .filter(|x| *x != '\"' && *x != '\\')
        .collect();

    let bytes = bytes.chunks(3)
        .map(|x| {
            if x.len() != 3 || x[0] != 'x' {
                return Err(());
            }

            let mut buf = String::new();
            buf.push(x[1]);
            buf.push(x[2]);

            u8::from_str_radix(&buf, 16).or(Err(()))
        }).collect();

    bytes
}


#[cfg(test)]
mod tests {
    use super::*;
    use ctrl;
    use Toolbox;

    #[inline]
    fn str_args(args: Vec<&str>) -> Arguments {
        args.into_iter()
            .map(|x| x.to_owned())
            .collect()
    }

    #[test]
    fn test_echo() {
        let mut sh = Shell::new(Toolbox::empty());
        sh.hotswap(ctrl::Interface::dummy());

        echo(&mut sh, str_args(vec!["echo", "foo"])).unwrap();
        echo(&mut sh, str_args(vec!["echo", "--", "bar", "asdf"])).unwrap();
        echo(&mut sh, str_args(vec!["echo"])).unwrap();
        echo(&mut sh, str_args(vec!["echo", "-x", "--yz"])).unwrap();
    }

    #[test]
    fn test_unhexify() {
        assert_eq!(vec![255], unhexify("\\xff").unwrap());
        assert_eq!(vec![255], unhexify("\\xFF").unwrap());
        assert_eq!(vec![255], unhexify("\"\\xff\"").unwrap());
        assert_eq!(vec![1, 2, 193, 251], unhexify("\\x01\\x02\\xc1\\xfb").unwrap());
        assert_eq!(vec![1, 2, 193, 251], unhexify("\"\\x01\\x02\\xc1\\xfb\"").unwrap());
        assert_eq!(vec![1, 2, 193, 251], unhexify("\\x01\\x02\\xc1\\xfb\"").unwrap());
    }
}
