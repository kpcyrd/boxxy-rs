use clap::{App, Arg, AppSettings};

use ::{Result, Shell, Arguments};
use error::ResultExt;

use nix;


pub fn fchdir(_sh: &mut Shell, args: Arguments) -> Result<()> {
    let matches = App::new("fds")
        .setting(AppSettings::DisableVersion)
        .arg(Arg::with_name("fd")
            .required(true)
        )
        .get_matches_from_safe(args)?;

    let fd: i32 = matches.value_of("fd").unwrap().parse()
        .chain_err(|| "Failed to parse fds")?;

    nix::unistd::fchdir(fd)
        .chain_err(|| "Failed to fchdir")?;

    Ok(())
}
