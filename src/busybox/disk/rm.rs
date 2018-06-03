use clap::{App, Arg, AppSettings};

use ::{Result, Shell, Arguments};

use std::fs;


pub fn rm(sh: &mut Shell, args: Arguments) -> Result<()> {
    let matches = App::new("rm")
        .setting(AppSettings::DisableVersion)
        .arg(Arg::with_name("path")
            .required(true)
            .multiple(true)
        )
        .arg(Arg::with_name("r").short("r"))
        .arg(Arg::with_name("f").short("f"))
        .get_matches_from_safe(args)?;

    let recursive = matches.occurrences_of("r") > 0;

    for path in matches.values_of("path").unwrap() {
        debug!("rm: {:?}", path);

        let result = if recursive {
            fs::remove_dir_all(path)
        } else {
            fs::remove_file(path)
        };

        if let Err(err) = result {
            shprintln!(sh, "rm: {:?}: {:?}", path, err);
        }
    }

    Ok(())
}
