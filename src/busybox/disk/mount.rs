use clap::{App, Arg, AppSettings};
use crate::{Shell, Arguments};
use crate::errors::*;

pub fn mount(sh: &mut Shell, args: Arguments) -> Result<()> {
    let matches = App::new("mount")
        .setting(AppSettings::DisableVersion)
        .arg(Arg::with_name("src")
            .required(true)
        )
        .arg(Arg::with_name("dest"))
        .arg(Arg::with_name("fstype")
            .short("t")
            .takes_value(true)
        )
        .get_matches_from_safe(args)?;

    let (src, dest) = match matches.value_of("dest") {
        Some(dest) => {
            let src = matches.value_of("src");
            (src, dest)
        },
        None => {
            let src = None;
            let dest = matches.value_of("src").unwrap();
            (src, dest)
        },
    };

    let fstype = matches.value_of("fstype");
    let flags = nix::mount::MsFlags::empty();

    let data: Option<&'static [u8]> = None;

    if let Err(err) = nix::mount::mount(src, dest, fstype, flags, data) {
        shprintln!(sh, "error: mount: {:?}", err);
    }

    Ok(())
}
