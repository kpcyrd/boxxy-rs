use clap::Parser;
use crate::errors::*;
use crate::{Shell, Arguments};

#[derive(Debug, Parser)]
#[clap(name = "fds")]
pub struct Args {
}

pub fn fds(sh: &mut Shell, args: Arguments) -> Result<()> {
    let _args = Args::try_parse_from(args)?;

    for i in close_fds::iter_open_fds(0) {
        shprintln!(sh, "{:?}", i);
    }

    Ok(())
}
