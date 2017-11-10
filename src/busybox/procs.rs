use clap::{App, Arg, AppSettings};
use base64;
use libc;

use ::{Result, Arguments};

use std::mem;
use std::process::Command;


pub fn echo(args: Arguments) -> Result {
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

    println!("{}", msg);

    Ok(())
}


pub fn jit(args: Arguments) -> Result {
    let matches = App::new("jit")
        .setting(AppSettings::DisableVersion)
        .arg(Arg::with_name("shellcode")
            .help("base64 encoded shellcode")
            .required(true)
        )
        .get_matches_from_safe(args)?;

    let shellcode = matches.value_of("shellcode").unwrap();
    let mut shellcode: Vec<u8> = base64::decode(shellcode).unwrap();

    const PAGE_SIZE: usize = 4096;

    let num_pages = 1;
    let size = num_pages * PAGE_SIZE;
    let mut page: *mut libc::c_void = unsafe { mem::uninitialized() };

    unsafe { libc::posix_memalign(&mut page, PAGE_SIZE, size) };
    unsafe { libc::mprotect(page, size, libc::PROT_READ | libc::PROT_WRITE) };
    unsafe { libc::memcpy(page, shellcode.as_mut_ptr() as *mut libc::c_void, shellcode.len()) };
    unsafe { libc::mprotect(page, size, libc::PROT_READ | libc::PROT_EXEC) };

    println!("shellcode: {:?} ({} bytes) \"{}\"", shellcode.as_ptr(), shellcode.len(), shellcode.iter()
        .fold(String::new(), |a, b| {
            a + &format!("\\x{:02X}", b)
        }));

    let shellcode = unsafe { mem::transmute::<*mut libc::c_void, extern fn() -> u32>(page) };

    shellcode();

    unsafe { libc::free(page) };

    Ok(())
}


pub fn exec(mut args: Arguments) -> Result {
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


#[cfg(test)]
mod tests {
    use super::*;

    #[inline]
    fn str_args(args: Vec<&str>) -> Arguments {
        args.into_iter()
            .map(|x| x.to_owned())
            .collect()
    }

    #[test]
    fn test_echo() {
        echo(str_args(vec!["echo", "foo"])).unwrap();
        echo(str_args(vec!["echo", "--", "bar", "asdf"])).unwrap();
        echo(str_args(vec!["echo"])).unwrap();
        echo(str_args(vec!["echo", "-x", "--yz"])).unwrap();
    }
}
