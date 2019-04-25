use clap::{App, Arg, AppSettings};
use libc;
use errno::errno;

use crate::{Result, Shell, ErrorKind, Arguments};


pub fn setgid(_sh: &mut Shell, args: Arguments) -> Result<()> {
    let matches = App::new("setgid")
        .setting(AppSettings::DisableVersion)
        .about("Call setgid(2)")
        .arg(Arg::with_name("gid")
            .required(true)
        )
        .get_matches_from_safe(args)?;

    let uid = matches.value_of("gid").unwrap().parse()?;

    let ret = unsafe { libc::setgid(uid) };

    if ret != 0 {
        let err = errno();
        Err(ErrorKind::Errno(err).into())
    } else {
        Ok(())
    }
}
