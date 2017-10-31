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
//! extern crate boxxy;
//! extern crate env_logger;
//!
//! fn stage1(args: Vec<String>) -> Result<(), boxxy::Error> {
//!     println!("init stage 1! {:?}", args);
//!     // your code here
//!     Ok(())
//! }
//!
//! fn stage2(args: Vec<String>) -> Result<(), boxxy::Error> {
//!     println!("init stage 2! {:?}", args);
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

use std::io;
use std::num;

pub mod busybox;
pub mod ffi;
pub mod shell;

pub use shell::{Shell, Toolbox};

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


/// Export function for FFI
#[no_mangle]
pub extern fn run_boxxy() {
    Shell::new(Toolbox::new()).run();
}
