use clap::{App, Arg, AppSettings};

use hyper;
use hyper::client::Client;
use hyper_rustls::HttpsConnector;
use url::Url;

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
    let https = HttpsConnector::new(4);
    let client: Client<_, hyper::Body> = Client::builder().build(https);

    #[allow(unused_assignments)]
    let (mut res, mut location) = (None, Some(url));

    let mut max_redirects = 12;

    loop {
        let original_url = location.unwrap();
        let url = original_url.clone();

        if verbose {
            shprintln!(sh, "requesting: {:?}", url);
        }

        let (inner_res, inner_location) = core.run(client.get(url).and_then(|res| {
            if verbose {
                shprintln!(sh, "{:?} {:?}", res.version(), res.status());

                for (key, value) in res.headers().iter() {
                    shprintln!(sh, "  {:?}; {:?}", key, value);
                }

                if output.is_none() {
                    shprintln!(sh, "");
                }
            }

            let mut next_location = None;
            if follow_location && res.status().is_redirection() {
                use http::header::LOCATION;

                if let Some(location) = res.headers().get(LOCATION) {
                    if verbose {
                        shprintln!(sh, "follow: {:?}", location);
                    }

                    // TODO: proper error handling
                    next_location = Some(resolve_redirect(&original_url, location.to_str().unwrap()).unwrap());
                }
            }

            (res.into_body().concat2(), futures::future::ok(next_location))
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

// TODO: proper error handling
fn resolve_redirect(current: &hyper::Uri, redirect: &str) -> Result<hyper::Uri> {
    let current = Url::parse(&current.to_string()).unwrap();
    let new_location = current.join(redirect).unwrap();

    let next = new_location.as_str().parse()?;
    Ok(next)
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

    #[test]
    fn test_relative_redirect() {
        assert_eq!(resolve_redirect(&"https://httpbin.org/x/y".parse().unwrap(), "/anything").unwrap(),
                        "https://httpbin.org/anything");
        assert_eq!(resolve_redirect(&"https://example.com/x/y".parse().unwrap(), "https://httpbin.org/anything").unwrap(),
                        "https://httpbin.org/anything");
    }
}
