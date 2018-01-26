use libc::{self, uid_t, gid_t};
use errno::errno;

use ::{Result, ErrorKind};


pub fn getuid() -> Result<uid_t> {
    let uid = unsafe { libc::getuid() };
    Ok(uid)
}


pub fn geteuid() -> Result<uid_t> {
    let euid = unsafe { libc::geteuid() };
    Ok(euid)
}



pub fn setuid(uid: uid_t) -> Result<()> {
    let ret = unsafe { libc::setuid(uid) };

    if ret != 0 {
        let err = errno();
        Err(ErrorKind::Errno(err).into())
    } else {
        Ok(())
    }
}


pub fn getgid() -> Result<uid_t> {
    let gid = unsafe { libc::getgid() };
    Ok(gid)
}


pub fn getegid() -> Result<uid_t> {
    let egid = unsafe { libc::getegid() };
    Ok(egid)
}


/// Get the supplemental groups.
///
/// ```
/// let groups = boxxy::ffi::getgroups().unwrap();
/// println!("groups={:?}", groups);
/// ```
pub fn getgroups() -> Result<Vec<gid_t>> {
    let size = 128;
    let mut gids: Vec<gid_t> = Vec::with_capacity(size as usize);

    let ret = unsafe { libc::getgroups(size, gids.as_mut_ptr()) };

    if ret < 0 {
        let err = errno();
        Err(ErrorKind::Errno(err).into())
    } else {
        let groups = (0..ret)
            .map(|i| unsafe { gids.get_unchecked(i as usize) }.to_owned())
            .collect();
        Ok(groups)
    }
}


