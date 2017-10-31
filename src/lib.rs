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


/// Crate a shell, returns a pointer.
#[no_mangle]
pub extern fn boxxy_init() -> *mut Shell {
    let shell = Shell::new(Toolbox::new());
    Box::into_raw(Box::new(shell))
}

/// Drop into a shell with default config.
#[no_mangle]
pub extern fn boxxy_run() {
    Shell::new(Toolbox::new()).run();
}

/// Extend the shell struct with additional commands.
#[no_mangle]
pub extern fn boxxy_with(target: *mut Shell, name: *const libc::c_char, ptr: ForeignCommand) {
    let name = unsafe {
        let bytes = std::ffi::CStr::from_ptr(name).to_bytes();
        String::from_utf8(bytes.to_vec()).ok().expect("Invalid UTF8 string").to_string()
    };

    debug!("registering: {:?} -> {:?}", name, ptr);
    unsafe { (&mut *target) }.insert(&name, ptr.into());
}

/// Start shell at specific pointer.
#[no_mangle]
pub extern fn boxxy_run_at(target: *mut Shell) {
    unsafe { (&mut *target) }.run()
}

/// Free memory.
#[no_mangle]
pub unsafe extern fn boxxy_free(target: *mut Shell) {
    if !target.is_null() {
        Box::from_raw(target);
    }
}
