use crate::errors::*;
use libc::{gid_t, uid_t};

/// Get the real uid, effective uid and saved uid.
///
/// ```
/// let (ruid, euid, suid) = boxxy::ffi::getresuid().unwrap();
/// println!("ruid={}, euid={}, suid={}", ruid, euid, suid);
/// ```
pub fn getresuid() -> Result<(uid_t, uid_t, uid_t)> {
    let mut ruid: uid_t = 0;
    let mut euid: uid_t = 0;
    let mut suid: uid_t = 0;

    let ret = unsafe { libc::getresuid(&mut ruid, &mut euid, &mut suid) };

    if ret != 0 {
        Err(errno())
    } else {
        Ok((ruid, euid, suid))
    }
}

/// Get the real gid, effective gid and saved gid.
///
/// ```
/// let (rgid, egid, sgid) = boxxy::ffi::getresgid().unwrap();
/// println!("rgid={}, egid={}, sgid={}", rgid, egid, sgid);
/// ```
pub fn getresgid() -> Result<(gid_t, gid_t, gid_t)> {
    let mut rgid: gid_t = 0;
    let mut egid: gid_t = 0;
    let mut sgid: gid_t = 0;

    let ret = unsafe { libc::getresgid(&mut rgid, &mut egid, &mut sgid) };

    if ret != 0 {
        Err(errno())
    } else {
        Ok((rgid, egid, sgid))
    }
}

/// Set the supplemental groups.
///
/// ```no_run
/// boxxy::ffi::setgroups(&[1,2,3]).unwrap();
/// ```
pub fn setgroups(groups: &[gid_t]) -> Result<()> {
    let ret = unsafe { libc::setgroups(groups.len(), groups.as_ptr()) };

    if ret < 0 {
        Err(errno())
    } else {
        Ok(())
    }
}
