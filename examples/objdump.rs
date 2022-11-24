use boxxy::errors::*;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
struct Args {
    /// Use hex instead of base64 encoding
    #[clap(short = 'x', long)]
    hex: bool,
    /// Path to binary that should be dumped
    path: PathBuf,
}

#[derive(Debug)]
enum Encoding {
    Hex,
    Base64,
}

impl Encoding {
    fn encode(&self, data: &[u8]) -> String {
        match &self {
            Encoding::Hex => data
                .iter()
                .fold(String::new(), |a, b| a + &format!("\\x{:02x}", b)),
            Encoding::Base64 => base64::encode(data),
        }
    }
}

fn main() -> Result<()> {
    let args = Args::parse();

    let encoding = if args.hex {
        Encoding::Hex
    } else {
        Encoding::Base64
    };

    let file = std::fs::File::open(&args.path)?;

    // Use a lazy i/o ElfStream so that we only read the ELF sections that we're interested in.
    // Makes this faster for large files where the .text section is small.
    let mut elf = elf::ElfStream::<elf::endian::AnyEndian, _>::open_stream(&file)?;

    let section = *elf
        .section_header_by_name(".text")?
        .expect("Could not find .text section");

    // Note: section_data() returns an optional compression context alongside the data
    // which informs if and how the data was compressed.
    // Typically, this is only done for debug info sections.
    let section_data = match elf.section_data(&section)? {
        (section_data, None) => section_data,
        _ => {
            panic!("Cannot dump compressed .text section");
        }
    };

    let shellcode = encoding.encode(section_data);
    println!("{}", shellcode);

    Ok(())
}
