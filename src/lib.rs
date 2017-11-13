//! This crate provides an interactive shell that can be used to provide a
//! basic debugger for sandboxes. It features a couple of builtin commands to
//! test eg. file access when external programs can't be used (chroots or
//! seccomp).
//!
//! It also accepts custom functions that can be included to step through
//! various stages of your sandbox.
//!
//! # Example
//!
//! ```
//! #[macro_use] extern crate boxxy;
//! extern crate env_logger;
//!
//! fn stage1(sh: &mut boxxy::Shell, args: Vec<String>) -> Result<(), boxxy::Error> {
//!     shprintln!(sh, "init stage 1! {:?}", args);
//!     // your code here
//!     Ok(())
//! }
//!
//! fn stage2(sh: &mut boxxy::Shell, args: Vec<String>) -> Result<(), boxxy::Error> {
//!     shprintln!(sh, "init stage 2! {:?}", args);
//!     // your code here
//!     Ok(())
//! }
//!
//! fn main() {
//!     env_logger::init().unwrap();
//!
//!     let toolbox = boxxy::Toolbox::new().with(vec![
//!             ("stage1", stage1),
//!             ("stage2", stage2),
//!         ]);
//!     boxxy::Shell::new(toolbox).run()
//! }
//! ```
extern crate rustyline;
#[macro_use] extern crate log;
extern crate clap;
extern crate libc;
extern crate errno;
extern crate regex;
extern crate nix;
extern crate base64;
#[macro_use] extern crate cfg_if;
extern crate rustls;
extern crate bufstream;
extern crate webpki;
extern crate crypto as rust_crypto;

#[cfg(feature="network")]
extern crate hyper;
#[cfg(feature="network")]
extern crate hyper_rustls;
#[cfg(feature="network")]
extern crate tokio_core;
#[cfg(feature="network")]
extern crate futures;

use std::io;
use std::num;

#[macro_use] mod macros;
pub mod busybox;
pub mod crypto;
pub mod ctrl;
pub mod ffi;
pub mod shell;

pub use shell::{Shell, Toolbox};
pub use shell::{Command, NativeCommand, ForeignCommand};

/// Result of builtin commands
pub type Result = std::result::Result<(), Error>;
/// Arguments passed to builtin commands
pub type Arguments = Vec<String>;

/// Possible errors during builtin commands
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
