use bufstream::BufStream;
use clap::Parser;
use crate::{Shell, Arguments};
use crate::errors::*;
use crate::ctrl::Interface;
use std::os::unix::net::UnixStream;
use std::path::PathBuf;

/// Connect to a unix domain socket and connect the interface to it
#[derive(Parser)]
#[clap(name = "ipcshell")]
struct Args {
    /// Explicitly execute main loop again
    #[clap(short = 'l', long = "loop")]
    run_loop: bool,
    /// Unix domain socket path
    path: PathBuf,
}

pub fn ipcshell(sh: &mut Shell, args: Arguments) -> Result<()> {
    let args = Args::try_parse_from(args)?;

    shprintln!(sh, "[*] connecting to {}...", args.path.display());
    let sock = UnixStream::connect(&args.path)?;
    shprintln!(sh, "[+] connected!");
    let sock = BufStream::new(sock);

    shprintln!(sh, "[*] see you on the other side...");
    sh.hotswap(Interface::Ipc(sock));
    shprintln!(sh, "[+] hot-swapped interface");

    if args.run_loop {
        sh.run();
    }

    Ok(())
}
