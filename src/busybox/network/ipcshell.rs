use clap::{App, Arg, AppSettings};
use bufstream::BufStream;

use ::{Result, Shell, Arguments};
use ctrl::Interface;

use std::os::unix::net::UnixStream;


pub fn ipcshell(sh: &mut Shell, args: Arguments) -> Result<()> {
    let matches = App::new("ipcshell")
        .setting(AppSettings::DisableVersion)
        .about("Connect to a unix domain socket and connect the interface to it")
        .arg(Arg::with_name("loop")
            .short("l")
            .long("loop")
            .help("Explicitly execute main loop again")
        )
        .arg(Arg::with_name("path")
            .required(true)
            .help("Unix domain socket path")
        )
        .get_matches_from_safe(args)?;

    let path = matches.value_of("path").unwrap();
    let run_loop = matches.occurrences_of("loop") > 0;

    shprintln!(sh, "[*] connecting to {}...", path);
    let sock = UnixStream::connect(&path)?;
    shprintln!(sh, "[+] connected!");
    let sock = BufStream::new(sock);

    shprintln!(sh, "[*] see you on the other side...");
    sh.hotswap(Interface::Ipc(sock));
    shprintln!(sh, "[+] hot-swapped interface");

    if run_loop {
        sh.run();
    }

    Ok(())
}
