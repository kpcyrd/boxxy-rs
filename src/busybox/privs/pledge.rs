use clap::{App, Arg, AppSettings};
use pledge::pledge as pledge_rs;

use ::{Result, Shell, Arguments};

pub fn pledge(_sh: &mut Shell, args: Arguments) -> Result<()> {
    let matches = App::new("pledge")
        .setting(AppSettings::DisableVersion)
        .arg(Arg::with_name("promises")
            .multiple(true)
        )
        .get_matches_from_safe(args)?;

    let promises = match matches.values_of("promises") {
        Some(promises) => promises.collect(),
        None => Vec::new(),
    };

    let mut promises = promises.iter()
                               .fold(String::new(), |a, b| a + &b + " ");
    promises.pop();

    pledge_rs(&promises)?;

    Ok(())
}
