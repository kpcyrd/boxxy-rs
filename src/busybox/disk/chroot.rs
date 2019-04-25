use clap::{App, Arg, AppSettings};
use libc;
use errno::errno;

use crate::{Result, Shell, ErrorKind, Arguments};

use std::ffi::CString;


pub fn chroot(_sh: &mut Shell, args: Arguments) -> Result<()> {
    let matches = App::new("chroot")
        .setting(AppSettings::DisableVersion)
        .arg(Arg::with_name("path")
            .required(true)
        )
        .get_matches_from_safe(args)?;

    let path = matches.value_of("path").unwrap();
    let path = CString::new(path).unwrap();

    let ret = unsafe { libc::chroot(path.as_ptr()) };

    if ret != 0 {
        let err = errno();
        Err(ErrorKind::Errno(err).into())
    } else {
        Ok(())
    }
}
