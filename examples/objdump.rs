extern crate elf;
extern crate base64;

use std::env;

fn main() {
    let path = env::args().nth(1).expect("couldn't find path argument");

    let file = elf::File::open_path(path).expect("couldn't open file");
    let section = file.get_section(".text").expect("couldn't find .text section");

    let shellcode = base64::encode(&section.data);
    println!("{}", shellcode);
}
