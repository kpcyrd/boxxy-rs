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
            Encoding::Hex => data.iter()
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

    let file = elf::File::open_path(&args.path)
        .map_err(|e| anyhow!("Couldn't open file: {:?}", e))?;
    let section = file.get_section(".text")
        .context("Couldn't find .text section")?;

    let shellcode = encoding.encode(&section.data);
    println!("{}", shellcode);

    Ok(())
}
