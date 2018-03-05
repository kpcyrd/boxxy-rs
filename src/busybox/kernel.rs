use clap::{App, AppSettings};
use kmod;

use ::{Result, Shell, Arguments};

pub fn lsmod(sh: &mut Shell, args: Arguments) -> Result<()> {
    let _matches = App::new("lsmod")
        .setting(AppSettings::DisableVersion)
        .get_matches_from_safe(args)?;

    let ctx = kmod::Context::new();

    for module in ctx.modules_loaded() {
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
