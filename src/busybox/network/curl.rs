use clap::{App, Arg, AppSettings};

use hyper;
use hyper_rustls::HttpsConnector;

use tokio_core::reactor;
use futures;
use futures::Stream;
use futures::future::Future;

use ::{Result, Shell, Arguments};

use std::fs::File;
use std::io::prelude::*;


pub fn curl(sh: &mut Shell, args: Arguments) -> Result<()> {
    let matches = App::new("curl")
        .setting(AppSettings::DisableVersion)
        .about("Poor mans curl")
        .arg(Arg::with_name("verbose")
            .short("v")
            .help("Verbose output")
        )
        .arg(Arg::with_name("output")
            .short("o")
            .takes_value(true)
            .help("Write response to file")
        )
        .arg(Arg::with_name("remote-name")
            .short("O")
            .help("Download file and use the remote filename")
        )
        .arg(Arg::with_name("location")
            .short("L")
            .help("Follow redirects")
        )
        .arg(Arg::with_name("url")
            .required(true)
            .help("Fetch this url")
        )
        .get_matches_from_safe(args)?;

    let verbose = matches.occurrences_of("verbose") > 0;
    let remote_name = matches.occurrences_of("remote-name") > 0;
    let follow_location = matches.occurrences_of("location") > 0;
    let mut output = matches.value_of("output").and_then(|x| Some(String::from(x)));
    // TODO: show error if != 200

    let url = matches.value_of("url").unwrap();
    let url = url.parse()?;

    if output.is_none() && remote_name {
        output = Some(filename_from_uri(&url));
    }

    let mut core = reactor::Core::new().unwrap();
    let client = hyper::Client::configure()
        .connector(HttpsConnector::new(4, &core.handle()))
        .build(&core.handle());

    #[allow(unused_assignments)]
    let (mut res, mut location) = (None, Some(url));

    let mut max_redirects = 12;

    loop {
        let url = location.unwrap();
        if verbose {
            shprintln!(sh, "requesting: {:?}", url);
        }

        let (inner_res, inner_location) = core.run(client.get(url).and_then(|res| {
            if verbose {
                for header in res.headers().iter() {
                    shprintln!(sh, "  {:?}; {:?}", header.name(), header.raw());
                }

                if output.is_none() {
                    shprintln!(sh, "");
                }
            }

            let mut next_location = None;
            if follow_location && res.status().is_redirection() {
                use hyper::header::Location;

                if let Some(location) = res.headers().get::<Location>() {
                    if verbose {
                        shprintln!(sh, "follow: {:?}", location);
                    }
                    next_location = Some(String::from(&location[..]).parse().unwrap());
                }
            }

            (res.body().concat2(), futures::future::ok(next_location))
        }))?;

        res = Some(inner_res);
        location = inner_location;

        if location.is_none() {
            break;
        }

        max_redirects -= 1;

        if max_redirects <= 0 {
            shprintln!(sh, "max redirects exceeded");
            break;
        }
    }

    let res = res.unwrap();

    match output {
        Some(path) => {
            let mut file = File::create(&path)?;
            // TODO: don't buffer the full response
            file.write_all(&res.to_vec())?;
            shprintln!(sh, "downloaded to: {:?}", path);
        },
        None => {
            // if printing to stdout
            let res = match String::from_utf8(res.to_vec()) {
                Ok(res) => format!("{:?}", res),
                Err(_) => format!("{:?}", res.to_vec()),
            };

            shprintln!(sh, "{}", res);
        }
    };

    Ok(())
}

fn filename_from_uri(uri: &hyper::Uri) -> String {
    let path = uri.path();

    if let Some(idx) = path.rfind('/') {
        let filename = &path[idx + 1..];

        if filename == "" {
            String::from("index.html")
        } else {
            String::from(filename)
        }
    } else {
        String::from("index.html")
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filename_from_uri() {
        assert_eq!(filename_from_uri(&"https://example.com".parse().unwrap()), "index.html");
        assert_eq!(filename_from_uri(&"https://example.com/".parse().unwrap()), "index.html");
        assert_eq!(filename_from_uri(&"https://example.com/foo/".parse().unwrap()), "index.html");
        assert_eq!(filename_from_uri(&"https://example.com/asdf/foo/".parse().unwrap()), "index.html");
        assert_eq!(filename_from_uri(&"https://example.com/foo/?a=1".parse().unwrap()), "index.html");
        assert_eq!(filename_from_uri(&"https://example.com/foo/?a=1#x".parse().unwrap()), "index.html");

        assert_eq!(filename_from_uri(&"https://example.com/foo.txz".parse().unwrap()), "foo.txz");
        assert_eq!(filename_from_uri(&"https://example.com/asdf/foo.txz".parse().unwrap()), "foo.txz");
        assert_eq!(filename_from_uri(&"https://example.com/foo.txz?a=1".parse().unwrap()), "foo.txz");
        assert_eq!(filename_from_uri(&"https://example.com/foo.txz?a=1#x".parse().unwrap()), "foo.txz");
    }
}
