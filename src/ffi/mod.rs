//! Abstractions of some unsafe functions.
use Error;
use std::ptr;
use std::ffi::CString;

pub mod exports;
pub use self::exports::*;

#[cfg(target_os="linux")]
#[path="linux.rs"]
mod native;

#[cfg(target_os="macos")]
#[path="macos.rs"]
mod native;

#[cfg(target_os="windows")]
#[path="windows.rs"]
mod native;

pub use self::native::*;

#[cfg(unix)]
mod unix;
#[cfg(unix)]
pub use self::unix::*;


#[derive(Debug)]
pub struct ForeignCommand(extern fn(usize, *const *const i8) -> i32);

impl ForeignCommand {
    #[inline]
    pub fn run(&self, args: Vec<String>) -> Result<(), Error> {
        let argc = args.len();

        let args: Vec<_> = args.into_iter()
            .map(|x| CString::new(x).unwrap())
            .collect();

        let mut argv: Vec<_> = args.iter()
            .map(|x| x.as_ptr())
            .collect();
        argv.push(ptr::null()); // execve compatibility

        self.0(argc, argv.as_ptr());
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use libc;

    #[test]
    #[cfg(target_os="linux")]
    fn test_getresuid() {
        let ruid1 = unsafe { libc::getuid() };
        let euid1 = unsafe { libc::geteuid() };

        let (ruid2, euid2, _) = getresuid().unwrap();

        assert_eq!((ruid1, euid1), (ruid2, euid2));
    }

    #[test]
    #[cfg(target_os="linux")]
    fn test_getresgid() {
        let rgid1 = unsafe { libc::getgid() };
        let egid1 = unsafe { libc::getegid() };

        let (rgid2, egid2, _) = getresgid().unwrap();

        assert_eq!((rgid1, egid1), (rgid2, egid2));
    }
}
