use clap::{App, Arg, AppSettings};
use libc;
use errno::errno;

use crate::{Result, Shell, ErrorKind, Arguments};


pub fn seteuid(_sh: &mut Shell, args: Arguments) -> Result<()> {
    let matches = App::new("seteuid")
        .setting(AppSettings::DisableVersion)
        .about("Call seteuid(2)")
        .arg(Arg::with_name("uid").required(true))
        .get_matches_from_safe(args)?;

    let uid = matches.value_of("uid").unwrap().parse()?;

    let ret = unsafe { libc::seteuid(uid) };

    if ret != 0 {
        let err = errno();
        Err(ErrorKind::Errno(err).into())
    } else {
        Ok(())
    }
}
