use clap::Parser;
use crate::{Result, Shell, Arguments};
use pledge::pledge as pledge_rs;

#[derive(Debug, Parser)]
#[clap(name = "pledge")]
pub struct Args {
    #[clap(short)]
    promises: Option<String>,
    #[clap(short)]
    exec_promises: Option<String>,
}

pub fn pledge(_sh: &mut Shell, args: Arguments) -> Result<()> {
    let args = Args::try_parse_from(args)?;

    let promises = args.promises.as_deref();
    let exec_promises = args.exec_promises.as_deref();

    pledge_rs(promises, exec_promises)?;

    Ok(())
}
