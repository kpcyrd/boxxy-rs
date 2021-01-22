use clap::{App, Arg, AppSettings};
use crate::{Shell, Arguments};
use crate::errors::*;

pub fn setresuid(_sh: &mut Shell, args: Arguments) -> Result<()> {
    let matches = App::new("setresuid")
        .setting(AppSettings::DisableVersion)
        .about("Call setresuid(2)")
        .arg(Arg::with_name("ruid").required(true))
        .arg(Arg::with_name("euid").required(true))
        .arg(Arg::with_name("suid").required(true))
        .get_matches_from_safe(args)?;

    let ruid = matches.value_of("ruid").unwrap().parse()?;
    let euid = matches.value_of("euid").unwrap().parse()?;
    let suid = matches.value_of("suid").unwrap().parse()?;

    let ret = unsafe { libc::setresuid(ruid, euid, suid) };

    if ret != 0 {
        Err(errno())
    } else {
        Ok(())
    }
}
