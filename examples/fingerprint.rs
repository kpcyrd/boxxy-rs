use anyhow::{Context, Result};
use clap::Parser;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub struct Args {
    path: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let buf = fs::read(&args.path).context("Failed to read certificate file")?;

    let cert = pem::parse(&buf).context("Failed to parse certificate as pem")?;

    let fingerprint = {
        let mut h = Sha256::new();
        h.update(cert.contents());
        h.finalize()
    };

    let fingerprint = base64::encode_config(fingerprint, base64::URL_SAFE_NO_PAD);
    println!("SHA256-{}", fingerprint);

    Ok(())
}
