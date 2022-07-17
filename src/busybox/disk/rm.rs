use clap::Parser;
use crate::{Shell, Arguments};
use crate::errors::*;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[clap(name = "rm")]
struct Args {
    /// Recursively delete folders
    #[clap(short, long)]
    recursive: bool,
    /// Paths to delete
    #[clap(required = true)]
    paths: Vec<PathBuf>,
}

pub fn rm(sh: &mut Shell, args: Arguments) -> Result<()> {
    let args = Args::try_parse_from(args)?;

    for path in &args.paths {
        debug!("rm: {:?}", path);

        let result = if args.recursive {
            fs::remove_dir_all(path)
        } else {
            fs::remove_file(path)
        };

        if let Err(err) = result {
            shprintln!(sh, "rm: {:?}: {:?}", path, err);
        }
    }

    Ok(())
}
