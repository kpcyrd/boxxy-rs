use clap::{App, Arg, AppSettings};
use crate::{Shell, Arguments};
use crate::errors::*;

pub fn setreuid(_sh: &mut Shell, args: Arguments) -> Result<()> {
    let matches = App::new("setreuid")
        .setting(AppSettings::DisableVersion)
        .about("Call setreuid(2)")
        .arg(Arg::with_name("ruid").required(true))
        .arg(Arg::with_name("euid").required(true))
        .get_matches_from_safe(args)?;

    let ruid = matches.value_of("ruid").unwrap().parse()?;
    let euid = matches.value_of("euid").unwrap().parse()?;

    let ret = unsafe { libc::setreuid(ruid, euid) };

    if ret != 0 {
        Err(errno())
    } else {
        Ok(())
    }
}
