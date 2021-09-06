use crate::errors::*;
use libc::gid_t;

/// Set the supplemental groups.
///
/// ```no_run
/// boxxy::ffi::setgroups(&[1,2,3]).unwrap();
/// ```
pub fn setgroups(groups: &[gid_t]) -> Result<()> {
    let ret = unsafe { libc::setgroups(groups.len() as i32, groups.as_ptr()) };

    if ret < 0 {
        let err = errno();
        Err(ErrorKind::Errno(err).into())
    } else {
        Ok(())
    }
}
