use clap::{App, Arg, AppSettings};
use crate::{Shell, Arguments};
use crate::errors::*;
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
        Err(errno())
    } else {
        Ok(())
    }
}
