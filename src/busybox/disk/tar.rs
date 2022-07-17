use clap::{App, Arg, AppSettings, ArgGroup};
use libflate::gzip;

use crate::errors::*;
use crate::shell::Shell;
use crate::Arguments;

use std::io;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;


#[derive(Debug)]
enum ArchiveReader {
    File(File),
    Gzip(Box<gzip::Decoder<File>>),
}

impl Read for ArchiveReader {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match *self {
            ArchiveReader::File(ref mut f) => f.read(buf),
            ArchiveReader::Gzip(ref mut f) => f.read(buf),
        }
    }
}

enum ArchiveWriter {
    File(File),
    Gzip(Box<gzip::Encoder<File>>),
}

impl ArchiveWriter {
    fn finish(self) -> Result<()> {
        match self {
            ArchiveWriter::File(_) => (),
            ArchiveWriter::Gzip(f) => {
                f.finish().into_result()?;
            },
        }
        Ok(())
    }
}

impl Write for ArchiveWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match *self {
            ArchiveWriter::File(ref mut f) => f.write(buf),
            ArchiveWriter::Gzip(ref mut f) => f.write(buf),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match *self {
            ArchiveWriter::File(ref mut f) => f.flush(),
            ArchiveWriter::Gzip(ref mut f) => f.flush(),
        }
    }
}

#[derive(Debug)]
enum Compression {
    Gzip,
    None,
}

impl Compression {
    #[inline]
    fn open(&self, path: &str) -> Result<ArchiveReader> {
        let file = File::open(path)?;
        match *self {
            Compression::Gzip => Ok(ArchiveReader::Gzip(Box::new(gzip::Decoder::new(file)?))),
            Compression::None => Ok(ArchiveReader::File(file)),
        }
    }

    #[inline]
    fn create(&self, path: &str) -> Result<ArchiveWriter> {
        let file = File::create(path)?;
        match *self {
            Compression::Gzip => Ok(ArchiveWriter::Gzip(Box::new(gzip::Encoder::new(file)?))),
            Compression::None => Ok(ArchiveWriter::File(file)),
        }
    }
}


pub fn tar(sh: &mut Shell, args: Arguments) -> Result<()> {
    let matches = App::new("tar")
        .setting(AppSettings::DisableVersion)
        .group(ArgGroup::with_name("action")
            .args(&["extract", "create"])
            .required(true)
        )
        .arg(Arg::with_name("extract")
            .short('x')
            .help("Exctract an archive")
        )
        .arg(Arg::with_name("create")
            .short('c')
            .help("Create an archive")
        )
        .arg(Arg::with_name("file")
            .short('f')
            .help("Dummy flag")
        )
        .arg(Arg::with_name("gz")
            .short('z')
            .help("Use gzip compression")
        )
        .arg(Arg::with_name("verbose")
            .short('v')
            .multiple(true)
            .help("Verbose output")
        )
        .arg(Arg::with_name("archive")
            .required(true)
            .help("Archive path")
        )
        .arg(Arg::with_name("path")
            .multiple(true)
        )
        .get_matches_from_safe(args)?;

    // TODO: -t
    let extract = matches.is_present("extract");
    let create = matches.is_present("create");
    let verbose = matches.occurrences_of("verbose");
    let gz = matches.is_present("gz");
    let archive = matches.value_of("archive").unwrap();

    let paths = match matches.values_of("path") {
        Some(paths) => paths.into_iter().collect(),
        None => vec![],
    };

    let compression = if gz {
        Compression::Gzip
    } else {
        Compression::None
    };

    if extract {
        let dest = match paths.len() {
            0 => ".",
            1 => paths[0],
            _ => bail!("too many paths"),
        };

        if verbose > 0 {
            shprintln!(sh, "extracting to {:?}", dest);
        }

        let file = compression.open(archive)?;
        let mut ar = tar::Archive::new(file);
        ar.set_preserve_permissions(true);
        ar.set_unpack_xattrs(true);
        ar.unpack(dest)?;
    } else if create {
        if paths.is_empty() {
            bail!("paths is required with create");
        }

        let mut file = compression.create(archive)?;
        {
            let mut tar = tar::Builder::new(&mut file);

            for path in paths {
                let path = Path::new(path);

                if path.is_dir() {
                    if verbose > 0 {
                        shprintln!(sh, "adding directory {:?}", path);
                    }

                    tar.append_dir_all(path, path)?;
                } else {
                    if verbose > 0 {
                        shprintln!(sh, "adding file {:?}", path);
                    }

                    tar.append_file(path, &mut File::open(path)?)?;
                }
            }

            tar.finish()?;
        }
        file.finish()?;
    }

    Ok(())
}
