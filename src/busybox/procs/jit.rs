use clap::{App, Arg, AppSettings};
use base64;
use libc;

use ::{Result, Shell, Arguments};

use std::mem;


pub fn jit(sh: &mut Shell, args: Arguments) -> Result<()> {
    let matches = App::new("jit")
        .setting(AppSettings::DisableVersion)
        .about("Execute shell code")
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
    let mut shellcode: Vec<u8> = if hex {
        unhexify(shellcode)?
    } else {
        base64::decode(shellcode)?
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

fn unhexify(input: &str) -> Result<Vec<u8>> {
    // the escape sequence parser translates "\xFF" to "xFF",
    // so work with that for now
    let bytes: Vec<char> = input.chars()
        .filter(|x| *x != '\"' && *x != '\\')
        .collect();

    bytes.chunks(3)
        .map(|x| {
            if x.len() != 3 || x[0] != 'x' {
                bail!("invalid byte")
            }

            let mut buf = String::new();
            buf.push(x[1]);
            buf.push(x[2]);

            let byte = u8::from_str_radix(&buf, 16)?;
            Ok(byte)
        }).collect()
}


#[cfg(test)]
mod tests {
    use super::*;

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
