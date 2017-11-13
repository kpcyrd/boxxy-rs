use libc::{self, gid_t};
use errno::errno;

use Error;

/// Set the supplemental groups.
///
/// ```no_run
/// boxxy::ffi::setgroups(vec![1,2,3]).unwrap();
/// ```
pub fn setgroups(groups: Vec<gid_t>) -> Result<(), Error> {
    let ret = unsafe { libc::setgroups(groups.len(), groups.as_ptr()) };

    if ret < 0 {
        let err = errno();
        Err(Error::Errno(err))
    } else {
        Ok(())
    }
}
