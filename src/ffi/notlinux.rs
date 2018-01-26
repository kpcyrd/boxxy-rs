use libc::{self, gid_t};
use errno::errno;

use ::{Result, ErrorKind};

/// Set the supplemental groups.
///
/// ```no_run
/// boxxy::ffi::setgroups(vec![1,2,3]).unwrap();
/// ```
pub fn setgroups(groups: Vec<gid_t>) -> Result<()> {
    let ret = unsafe { libc::setgroups(groups.len() as i32, groups.as_ptr()) };

    if ret < 0 {
        let err = errno();
        Err(ErrorKind::Errno(err).into())
    } else {
        Ok(())
    }
}
