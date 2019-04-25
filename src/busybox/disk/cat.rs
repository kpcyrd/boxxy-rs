use clap::{App, Arg, AppSettings};

use crate::{Result, Shell, Arguments};

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;


pub fn cat(sh: &mut Shell, args: Arguments) -> Result<()> {
    let matches = App::new("cat")
        .setting(AppSettings::DisableVersion)
        .arg(Arg::with_name("path")
            .multiple(true)
            .required(true)
        )
        .get_matches_from_safe(args)?;

    for path in matches.values_of("path").unwrap() {
        debug!("cat: {:?}", path);

        match File::open(path) {
            Ok(file) => {
                let reader = BufReader::new(file);
                for line in reader.lines() {
                    shprintln!(sh, "{:?}", line);
                }
            },
            Err(err) => {
                shprintln!(sh, "error: {:?}", err);
            },
        };
    }

    Ok(())
}
