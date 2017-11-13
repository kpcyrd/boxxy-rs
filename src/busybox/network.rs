use clap::{App, Arg, AppSettings};

use hyper;
// use hyper_rustls;
use hyper_rustls::HttpsConnector;
// use tokio_core;
// use futures;

use tokio_core::reactor;
use futures::Stream;
use futures::future::Future;

// use ::{Result, Error, Arguments};
use ::{Result, Shell, Arguments};


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
