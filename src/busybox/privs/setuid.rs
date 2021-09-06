use clap::{App, Arg, AppSettings};
use crate::{Shell, Arguments};
use crate::errors::*;
use crate::ffi;

pub fn setuid(_sh: &mut Shell, args: Arguments) -> Result<()> {
    let matches = App::new("setuid")
        .setting(AppSettings::DisableVersion)
        .about("Call setuid(2)")
        .arg(Arg::with_name("uid").required(true))
        .get_matches_from_safe(args)?;

    let uid = matches.value_of("uid").unwrap().parse()?;

    ffi::setuid(uid)
}
