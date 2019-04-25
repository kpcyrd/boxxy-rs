use clap::{App, Arg, AppSettings};
use libc::gid_t;

use crate::{Result, Shell, Arguments};
use crate::ffi;

use std::result;


pub fn setgroups(_sh: &mut Shell, args: Arguments) -> Result<()> {
    let matches = App::new("setgroups")
        .setting(AppSettings::DisableVersion)
        .about("Call setgroups(2)")
        .arg(Arg::with_name("group")
            .multiple(true)
            .help("The groups that should be set")
        )
        .get_matches_from_safe(args)?;

    let groups = match matches.values_of("group") {
        Some(groups) => groups
            .map(|x| x.parse())
            .collect::<result::Result<Vec<gid_t>, _>>()?,
        None => Vec::new(),
    };

    ffi::setgroups(&groups)?;

    Ok(())
}
