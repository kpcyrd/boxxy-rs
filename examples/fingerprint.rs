use anyhow::{Context, Result};
use sha2::{Sha256, Digest};
use std::fs;
use std::path::PathBuf;
use structopt::{StructOpt, clap::AppSettings};

#[derive(Debug, StructOpt)]
#[structopt(global_settings = &[AppSettings::ColoredHelp])]
pub struct Args {
    path: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::from_args();

    let buf = fs::read(&args.path)
        .context("Failed to read certificate file")?;

    let cert = pem::parse(&buf)
        .context("Failed to parse certificate as pem")?;

    let fingerprint = {
        let mut h = Sha256::new();
        h.update(&cert.contents);
        h.finalize()
    };

    let fingerprint = base64::encode_config(&fingerprint, base64::URL_SAFE_NO_PAD);
    println!("SHA256-{}", fingerprint);

    Ok(())
}
