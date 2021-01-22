extern crate rustls;
extern crate sha2;
extern crate base64;

use std::env;
use std::fs::File;
use std::io::BufReader;

use sha2::{Sha256, Digest};
use rustls::internal::pemfile;


fn main() {
    let path = env::args().collect::<Vec<String>>().remove(1);

    let f = File::open(&path).unwrap();
    let mut reader = BufReader::new(f);
    let certs = pemfile::certs(&mut reader).unwrap();

    for cert in certs {
        let fingerprint = {
            let mut h = Sha256::new();
            h.update(&cert.0);
            h.finalize()
        };

        let fingerprint = base64::encode_config(&fingerprint, base64::URL_SAFE_NO_PAD);
        println!("SHA256-{}", fingerprint);
    }
}
