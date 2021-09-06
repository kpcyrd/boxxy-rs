use clap::{App, Arg, AppSettings};
use crate::{Shell, Arguments};
use crate::errors::*;

pub fn setresgid(_sh: &mut Shell, args: Arguments) -> Result<()> {
    let matches = App::new("setresgid")
        .setting(AppSettings::DisableVersion)
        .about("Call setresgid(2)")
        .arg(Arg::with_name("rgid").required(true))
        .arg(Arg::with_name("egid").required(true))
        .arg(Arg::with_name("sgid").required(true))
        .get_matches_from_safe(args)?;

    let rgid = matches.value_of("rgid").unwrap().parse()?;
    let egid = matches.value_of("egid").unwrap().parse()?;
    let sgid = matches.value_of("sgid").unwrap().parse()?;

    let ret = unsafe { libc::setresgid(rgid, egid, sgid) };

    if ret != 0 {
        Err(errno())
    } else {
        Ok(())
    }
}
