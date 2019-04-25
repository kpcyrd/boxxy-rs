use clap::{App, Arg, AppSettings};
use regex::Regex;

use crate::{Result, Shell, Arguments};

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;


pub fn grep(sh: &mut Shell, args: Arguments) -> Result<()> {

    // TODO: -i
    // TODO: -r
    // TODO: -n

    let matches = App::new("grep")
        .setting(AppSettings::DisableVersion)
        .arg(Arg::with_name("pattern")
            .required(true)
        )
        .arg(Arg::with_name("path")
            .multiple(true)
            .required(true)
        )
        .get_matches_from_safe(args)?;

    let pattern = matches.value_of("pattern").unwrap();
    let pattern = Regex::new(pattern)?;

    for path in matches.values_of("path").unwrap() {
        debug!("grep: {:?}", path);

        match File::open(path) {
            Ok(file) => {
                let reader = BufReader::new(file);
                for line in reader.lines() {
                    let line = line?;

                    if pattern.is_match(&line) {
                        shprintln!(sh, "{:?}", line);
                    }
                }
            },
            Err(err) => {
                shprintln!(sh, "error: {:?}", err);
            },
        };
    }

    Ok(())
}
