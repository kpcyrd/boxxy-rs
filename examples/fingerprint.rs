extern crate rustls;
extern crate crypto;
extern crate base64;

use std::env;
use std::fs::File;
use std::io::BufReader;

use std::iter::repeat;
use crypto::digest::Digest;
use rustls::internal::pemfile;

fn main() {
    let path = env::args().collect::<Vec<String>>().remove(1);

    let f = File::open(&path).unwrap();
    let mut reader = BufReader::new(f);
    let certs = pemfile::certs(&mut reader).unwrap();

    for cert in certs {
        let fingerprint = {
            let mut h = crypto::sha2::Sha256::new();
            h.input(&cert.0);

            let mut buf: Vec<u8> = repeat(0).take((h.output_bits()+7)/8).collect();
            h.result(&mut buf);
            buf
        };

        let fingerprint = base64::encode(&fingerprint);
        println!("SHA256:{}", fingerprint);

        break;
    }
}
