use pledge::pledge as pledge_rs;
use structopt::{StructOpt, clap::AppSettings};

use crate::{Result, Shell, Arguments};

#[derive(Debug, StructOpt)]
#[structopt(global_settings = &[AppSettings::ColoredHelp])]
pub struct Args {
    #[structopt(short)]
    promises: Option<String>,
    #[structopt(short)]
    exec_promises: Option<String>,
}

pub fn pledge(_sh: &mut Shell, args: Arguments) -> Result<()> {
    let args = Args::from_iter_safe(args)?;

    let promises = args.promises.as_deref();
    let exec_promises = args.exec_promises.as_deref();

    pledge_rs(promises, exec_promises)?;

    Ok(())
}
