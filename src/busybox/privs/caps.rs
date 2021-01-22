use caps::{Capability, CapSet};
use clap::{App, Arg, AppSettings, ArgGroup};
use crate::{Shell, Arguments};
use crate::errors::*;
use std::result;
use std::str::FromStr;
use std::collections::HashSet;


pub fn caps(sh: &mut Shell, args: Arguments) -> Result<()> {
    let matches = App::new("caps")
        .setting(AppSettings::DisableVersion)
        .group(ArgGroup::with_name("capset")
            .args(&["effective", "bounding", "inheritable", "ambient"])
        )
        .arg(Arg::with_name("effective")
                    .short("e")
                    .long("effective")
                    .help("Operate on the effective capset instead of the permitted capset"))
        .arg(Arg::with_name("bounding")
                    .short("b")
                    .long("bounding")
                    .help("Operate on the bounding capset instead of the permitted capset"))
        .arg(Arg::with_name("inheritable")
                    .short("i")
                    .long("inheritable")
                    .help("Operate on the inheritable capset instead of the permitted capset"))
        .arg(Arg::with_name("ambient")
                    .short("a")
                    .long("ambient")
                    .help("Operate on the ambient capset instead of the permitted capset"))
        .arg(Arg::with_name("clear")
                    .short("c")
                    .long("clear")
                    .help("Clear all capabilities"))
        .arg(Arg::with_name("drop")
                    .short("d")
                    .long("drop")
                    .help("Drop specific capabilities"))
        .arg(Arg::with_name("raise")
                    .short("r")
                    .long("raise")
                    .help("Add capabilities to capability set"))
        .arg(Arg::with_name("set")
                    .short("s")
                    .long("set")
                    .help("Set the capability set"))
        .arg(Arg::with_name("capabilities")
            .multiple(true)
        )
        .get_matches_from_safe(args)?;

    let clear = matches.occurrences_of("clear") > 0;
    let drop = matches.occurrences_of("drop") > 0;
    let raise = matches.occurrences_of("raise") > 0;
    let set = matches.occurrences_of("set") > 0;

    let capabilities = match matches.values_of("capabilities") {
        Some(caps) => caps.map(|c| Capability::from_str(c))
                          .collect::<result::Result<_, _>>()?,
        None => HashSet::new(),
    };

    let capset = if matches.is_present("effective") {
        CapSet::Effective
    } else if matches.is_present("bounding") {
        CapSet::Bounding
    } else if matches.is_present("inheritable") {
        CapSet::Inheritable
    } else if matches.is_present("ambient") {
        CapSet::Ambient
    } else {
        CapSet::Permitted
    };

    if clear {
        info!("caps(clear)");
        caps::clear(None, capset)?;
    } else if drop {
        for cap in capabilities {
            info!("caps(drop): {:?}", cap);
            caps::drop(None, capset, cap)?;
        }
    } else if raise {
        for cap in capabilities {
            info!("caps(raise): {:?}", cap);
            caps::raise(None, capset, cap)?;
        }
    } else if set {
        info!("caps(set): {:?}", capabilities);
        caps::set(None, capset, &capabilities)?;
    } else {
        let caps = caps::read(None, capset)?;
        shprintln!(sh, "{:?}", caps);
    }

    Ok(())
}
