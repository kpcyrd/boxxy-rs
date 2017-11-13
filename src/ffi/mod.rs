//! Abstractions of some unsafe functions.
use libc::{self, uid_t, gid_t};
use errno::errno;

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

pub use self::native::*;


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



pub fn getuid() -> Result<uid_t, Error> {
    let uid = unsafe { libc::getuid() };
    Ok(uid)
}


pub fn geteuid() -> Result<uid_t, Error> {
    let euid = unsafe { libc::geteuid() };
    Ok(euid)
}


pub fn setuid(uid: uid_t) -> Result<(), Error> {
    let ret = unsafe { libc::setuid(uid) };

    if ret != 0 {
        let err = errno();
        Err(Error::Errno(err))
    } else {
        Ok(())
    }
}


/// Get the real uid, effective uid and saved uid.
///
/// ```
/// let (ruid, euid, suid) = boxxy::ffi::getresuid().unwrap();
/// println!("ruid={}, euid={}, suid={}", ruid, euid, suid);
/// ```
#[cfg(target_os="linux")]
pub fn getresuid() -> Result<(uid_t, uid_t, uid_t), Error> {
    let mut ruid: uid_t = 0;
    let mut euid: uid_t = 0;
    let mut suid: uid_t = 0;

    let ret = unsafe { libc::getresuid(&mut ruid, &mut euid, &mut suid) };

    if ret != 0 {
        let err = errno();
        Err(Error::Errno(err))
    } else {
        Ok((ruid, euid, suid))
    }
}


pub fn getgid() -> Result<uid_t, Error> {
    let gid = unsafe { libc::getgid() };
    Ok(gid)
}


pub fn getegid() -> Result<uid_t, Error> {
    let egid = unsafe { libc::getegid() };
    Ok(egid)
}

/// Get the real gid, effective gid and saved gid.
///
/// ```
/// let (rgid, egid, sgid) = boxxy::ffi::getresgid().unwrap();
/// println!("rgid={}, egid={}, sgid={}", rgid, egid, sgid);
/// ```
#[cfg(target_os="linux")]
pub fn getresgid() -> Result<(gid_t, gid_t, gid_t), Error> {
    let mut rgid: gid_t = 0;
    let mut egid: gid_t = 0;
    let mut sgid: gid_t = 0;

    let ret = unsafe { libc::getresgid(&mut rgid, &mut egid, &mut sgid) };

    if ret != 0 {
        let err = errno();
        Err(Error::Errno(err))
    } else {
        Ok((rgid, egid, sgid))
    }
}


/// Get the supplemental groups.
///
/// ```
/// let groups = boxxy::ffi::getgroups().unwrap();
/// println!("groups={:?}", groups);
/// ```
pub fn getgroups() -> Result<Vec<gid_t>, Error> {
    let size = 128;
    let mut gids: Vec<gid_t> = Vec::with_capacity(size as usize);

    let ret = unsafe { libc::getgroups(size, gids.as_mut_ptr()) };

    if ret < 0 {
        let err = errno();
        Err(Error::Errno(err))
    } else {
        let groups = (0..ret)
            .map(|i| unsafe { gids.get_unchecked(i as usize) }.to_owned())
            .collect();
        Ok(groups)
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
