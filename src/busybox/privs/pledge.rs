use clap::{App, Arg, AppSettings};
use pledge::pledge as pledge_rs;

use ::{Result, Shell, Arguments};

pub fn pledge(_sh: &mut Shell, args: Arguments) -> Result<()> {
    let matches = App::new("pledge")
        .setting(AppSettings::DisableVersion)
        .arg(Arg::with_name("promises")
            .required(true)
        )
        .get_matches_from_safe(args)?;

    let promises = matches.value_of("promises").unwrap();
    pledge_rs(promises)?;

    Ok(())
}
