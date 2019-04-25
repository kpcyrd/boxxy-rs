use clap::{App, Arg, AppSettings};

use crate::{Result, Shell, Arguments};

use std::fs;


pub fn mkdir(_sh: &mut Shell, args: Arguments) -> Result<()> {
    let matches = App::new("mkdir")
        .setting(AppSettings::DisableVersion)
        .arg(Arg::with_name("directory")
            .required(true)
        )
        .arg(Arg::with_name("parents")
            .short("p")
            .long("parents")
        )
        .get_matches_from_safe(args)?;

    let directory = matches.value_of("directory").unwrap();
    let parents = matches.occurrences_of("parents") > 0;

    if parents {
        fs::create_dir_all(directory)?;
    } else {
        fs::create_dir(directory)?;
    }

    Ok(())
}
