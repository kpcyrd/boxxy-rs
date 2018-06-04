use clap::{App, Arg, AppSettings};

use ::{Result, Shell, Arguments};

use std::env;


pub fn cd(_sh: &mut Shell, args: Arguments) -> Result<()> {
    let matches = App::new("cd")
        .setting(AppSettings::DisableVersion)
        .arg(Arg::with_name("path")
            .required(true)
        )
        .get_matches_from_safe(args)?;

    let path = matches.value_of("path").unwrap();

    env::set_current_dir(&path)?;

    Ok(())
}
