use clap::{App, Arg, AppSettings};
use crate::{Shell, Arguments};
use crate::errors::*;

pub fn fchdir(_sh: &mut Shell, args: Arguments) -> Result<()> {
    let matches = App::new("fds")
        .setting(AppSettings::DisableVersion)
        .arg(Arg::with_name("fd")
            .required(true)
        )
        .get_matches_from_safe(args)?;

    let fd: i32 = matches.value_of("fd").unwrap().parse()
        .context("Failed to parse fds")?;

    nix::unistd::fchdir(fd)
        .context("Failed to fchdir")?;

    Ok(())
}
