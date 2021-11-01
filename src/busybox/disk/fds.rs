use crate::{Shell, Arguments};
use crate::errors::*;
use structopt::{StructOpt, clap::AppSettings};

#[derive(Debug, StructOpt)]
#[structopt(global_settings = &[AppSettings::ColoredHelp])]
pub struct Args {
}

pub fn fds(sh: &mut Shell, args: Arguments) -> Result<()> {
    let _args = Args::from_iter_safe(args)?;

    for i in close_fds::iter_open_fds(0) {
        shprintln!(sh, "{:?}", i);
    }

    Ok(())
}
