use libc::{self, uid_t, gid_t};
use errno::errno;

use Error;


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


pub fn getgid() -> Result<uid_t, Error> {
    let gid = unsafe { libc::getgid() };
    Ok(gid)
}


pub fn getegid() -> Result<uid_t, Error> {
    let egid = unsafe { libc::getegid() };
    Ok(egid)
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


