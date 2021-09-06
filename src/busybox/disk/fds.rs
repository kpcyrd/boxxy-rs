use clap::{App, Arg, AppSettings};
use crate::{Shell, Arguments};
use crate::errors::*;

pub fn fds(sh: &mut Shell, args: Arguments) -> Result<()> {
    let matches = App::new("fds")
        .setting(AppSettings::DisableVersion)
        .arg(Arg::with_name("max-fds")
            .default_value("65535")
        )
        .get_matches_from_safe(args)?;

    let max_fds: i32 = matches.value_of("max-fds").unwrap().parse()
        .context("Failed to parse max-fds")?;

    for i in 0..max_fds {
        if let Ok(fd) = nix::unistd::dup(i) {
            shprintln!(sh, "{:?}", i);
            // close fd again
            nix::unistd::close(fd).ok();
        }
    }

    Ok(())
}
