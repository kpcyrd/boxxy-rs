extern crate elf;
extern crate base64;
extern crate clap;

use clap::{App, Arg, AppSettings};

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
    let hex = matches.occurrences_of("hex") > 0;

    let file = elf::File::open_path(path).expect("couldn't open file");
    let section = file.get_section(".text").expect("couldn't find .text section");

    let shellcode = {
        if hex {
            section.data.iter()
                .fold(String::new(), |a, b| a + &format!("\\x{:02x}", b))
        } else {
            base64::encode(&section.data)
        }
    };
    println!("{}", shellcode);
}
