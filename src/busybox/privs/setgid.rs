use clap::{App, Arg, AppSettings};
use crate::{Shell, Arguments};
use crate::errors::*;

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
        Err(errno())
    } else {
        Ok(())
    }
}
