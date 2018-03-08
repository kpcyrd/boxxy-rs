use clap::{App, Arg, AppSettings};
use kmod;

use ::{Result, Shell, Arguments};

use std::fs;


pub fn lsmod(sh: &mut Shell, args: Arguments) -> Result<()> {
    let _matches = App::new("lsmod")
        .setting(AppSettings::DisableVersion)
        .get_matches_from_safe(args)?;

    let ctx = kmod::Context::new()?;

    for module in ctx.modules_loaded()? {
        let name = module.name();
        let refcount = module.refcount();
        let size = module.size();

        let holders: Vec<_> = module.holders()
                                .map(|x| x.name())
                                .collect();

        shprintln!(sh, "{:<19} {:8}  {} {:?}", name, size, refcount, holders);
    }

    Ok(())
}

pub fn insmod(_sh: &mut Shell, args: Arguments) -> Result<()> {
    let matches = App::new("insmod")
        .setting(AppSettings::DisableVersion)
        .arg(Arg::with_name("module")
            .required(true)
        )
        .arg(Arg::with_name("args")
            .multiple(true)
        )
        .get_matches_from_safe(args)?;

    let filename = matches.value_of("module").unwrap();
    let args = match matches.values_of("args") {
        Some(args) => args
            .map(|x| x.to_owned())
            .collect(),
        None => Vec::new(),
    };

    let ctx = kmod::Context::new()?;
    let module = ctx.module_new_from_path(&filename)?;
    info!("got module: {:?}", module.name());
    module.insert_module(0, args)?;

    Ok(())
}

pub fn rmmod(_sh: &mut Shell, args: Arguments) -> Result<()> {
    let matches = App::new("rmmod")
        .setting(AppSettings::DisableVersion)
        .arg(Arg::with_name("module")
            .required(true)
        )
        .get_matches_from_safe(args)?;

    let filename = matches.value_of("module").unwrap();
    let ctx = kmod::Context::new()?;

    let module = match fs::metadata(&filename) {
        Ok(_) => ctx.module_new_from_path(&filename)?,
        Err(_) => ctx.module_new_from_name(&filename)?,
    };

    info!("got module: {:?}", module.name());

    module.remove_module(0)?;

    Ok(())
}
