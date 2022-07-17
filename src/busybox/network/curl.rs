use clap::{App, Arg, AppSettings};
use crate::{Shell, Arguments};
use crate::errors::*;
use reqwest::{ClientBuilder, Request, Method};
use tokio::fs::File;
use std::io::prelude::*;
use futures_util::StreamExt;
use tokio::io::AsyncWriteExt;

#[tokio::main(flavor="current_thread")]
pub async fn curl(sh: &mut Shell, args: Arguments) -> Result<()> {

    let matches = App::new("curl")
        .setting(AppSettings::DisableVersion)
        .about("Poor mans curl")
        .arg(Arg::with_name("verbose")
            .short('v')
            .help("Verbose output")
        )
        .arg(Arg::with_name("output")
            .short('o')
            .takes_value(true)
            .help("Write response to file")
        )
        .arg(Arg::with_name("remote-name")
            .short('O')
            .help("Download file and use the remote filename")
        )
        .arg(Arg::with_name("location")
            .short('L')
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
    let mut output = matches.value_of("output").map(String::from);
    // TODO: show error if != 200

    let url = matches.value_of("url").unwrap();
    let url = url.parse()?;

    if output.is_none() && remote_name {
        output = Some(filename_from_uri(&url));
    }

    let builder = ClientBuilder::new()
        .use_rustls_tls();

    let builder = if follow_location {
        builder.redirect(reqwest::redirect::Policy::limited(12))
    } else {
        builder
    };

    let client = builder.build()?;
    let req = Request::new(Method::GET, url);
    let resp = client.execute(req).await?;

    if verbose {
        shprintln!(sh, "{:?} {:?}", resp.version(), resp.status());

        for (key, value) in resp.headers().iter() {
            shprintln!(sh, "  {:?}; {:?}", key, value);
        }

        if output.is_none() {
            shprintln!(sh, "");
        }
    }

    let mut output: Option<File> = if let Some(path) = output {
        Some(File::create(&path).await?)
    } else {
        None
    };

    let mut stream = resp.bytes_stream();
    while let Some(item) = stream.next().await {
        let chunk = item?;

        if let Some(file) = output.as_mut() {
            file.write_all(&chunk).await?;
        } else {
            sh.write_all(&chunk)?;
        }
    }

    Ok(())
}

fn filename_from_uri(uri: &reqwest::Url) -> String {
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
