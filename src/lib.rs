extern crate rustyline;
#[macro_use] extern crate log;
extern crate clap;
extern crate libc;
extern crate errno;
extern crate regex;
extern crate nix;

use std::io;
use std::num;

pub mod busybox;
pub mod ffi;
pub mod shell;

pub use shell::{Shell, Toolbox};

pub type Result = std::result::Result<(), Error>;
pub type Arguments = Vec<String>;

#[derive(Debug)]
pub enum Error {
    Args(clap::Error),
    Io(io::Error),
    Errno(errno::Errno),
    InvalidNum(std::num::ParseIntError),
    InvalidRegex(regex::Error),
}

impl From<clap::Error> for Error {
    fn from(err: clap::Error) -> Error {
        Error::Args(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<num::ParseIntError> for Error {
    fn from(err: num::ParseIntError) -> Error {
        Error::InvalidNum(err)
    }
}

impl From<regex::Error> for Error {
    fn from(err: regex::Error) -> Error {
        Error::InvalidRegex(err)
    }
}


#[no_mangle]
pub extern fn run_boxxy() {
    Shell::new(Toolbox::new()).run();
}
