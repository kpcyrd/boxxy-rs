use clap::{App, SubCommand, AppSettings};
use caps::securebits;

use crate::{Result, Shell, Arguments};


pub fn keepcaps(sh: &mut Shell, args: Arguments) -> Result<()> {
    let matches = App::new("caps")
        .setting(AppSettings::DisableVersion)
        .subcommand(SubCommand::with_name("on"))
        .subcommand(SubCommand::with_name("off"))
        .get_matches_from_safe(args)?;

    if matches.subcommand_matches("on").is_some() {
        securebits::set_keepcaps(true)?;
    } else if matches.subcommand_matches("off").is_some() {
        securebits::set_keepcaps(false)?;
    } else {
        shprintln!(sh, "{}", if securebits::has_keepcaps()? {
            "on"
        } else {
            "off"
        });
    }

    Ok(())
}
