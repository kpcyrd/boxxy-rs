// use libc::{self, uid_t, gid_t};
use libc::{self, gid_t};
use errno::errno;

use Error;


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


pub fn setgroups(groups: Vec<gid_t>) -> Result<(), Error> {
    let ret = unsafe { libc::setgroups(groups.len(), groups.as_ptr()) };

    if ret < 0 {
        let err = errno();
        Err(Error::Errno(err))
    } else {
        Ok(())
    }
}


