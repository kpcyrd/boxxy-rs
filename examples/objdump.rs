extern crate elf;
extern crate base64;
extern crate clap;

use clap::{App, Arg, AppSettings};

#[derive(Debug)]
enum Encoding {
    Hex,
    Base64,
}

#[inline]
fn encode(data: &[u8], encoding: Encoding) -> String {
    match encoding {
        Encoding::Hex => data.iter()
                .fold(String::new(), |a, b| a + &format!("\\x{:02x}", b)),
        Encoding::Base64 => base64::encode(data),
    }
}

fn main() {
    let matches = App::new("objdump")
        .setting(AppSettings::DisableVersion)
        .arg(Arg::with_name("hex")
            .help("use hex encoding")
            .short("x")
        )
        .arg(Arg::with_name("path")
            .help("binary path")
            .required(true)
        )
        .get_matches();

    let path = matches.value_of("path").unwrap();

    let encoding = match matches.occurrences_of("hex") > 0 {
        true => Encoding::Hex,
        false => Encoding::Base64,
    };

    let file = elf::File::open_path(path).expect("couldn't open file");
    let section = file.get_section(".text").expect("couldn't find .text section");

    let shellcode = encode(&section.data, encoding);
    println!("{}", shellcode);
}
