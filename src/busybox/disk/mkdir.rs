use clap::Parser;
use crate::{Shell, Arguments};
use crate::errors::*;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[clap(name = "mkdir")]
struct Args {
    /// Create parent directories too
    #[clap(short, long)]
    parents: bool,
    /// Directory to create
    directory: PathBuf,
}

pub fn mkdir(_sh: &mut Shell, args: Arguments) -> Result<()> {
    let args = Args::try_parse_from(args)?;

    if args.parents {
        fs::create_dir_all(args.directory)?;
    } else {
        fs::create_dir(args.directory)?;
    }

    Ok(())
}
