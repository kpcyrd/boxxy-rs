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
//!     env_logger::init();
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
#[macro_use] extern crate error_chain;
#[macro_use] extern crate cfg_if;

#[cfg(all(target_os="linux", target_arch="x86_64"))]
extern crate caps;

#[cfg(feature="network")]
extern crate rustls;
#[cfg(feature="network")]
extern crate bufstream;
#[cfg(feature="network")]
extern crate webpki;
#[cfg(feature="network")]
extern crate crypto as rust_crypto;

#[cfg(feature="network")]
extern crate hyper;
#[cfg(feature="network")]
extern crate hyper_rustls;
#[cfg(feature="network")]
extern crate tokio_core;
#[cfg(feature="network")]
extern crate futures;

mod error {
    use clap;
    use regex;
    use errno;

    #[cfg(feature="network")]
    use hyper;

    #[cfg(all(target_os="linux", target_arch="x86_64"))]
    use caps;

    use std::io;
    use std::num;

    error_chain! {
        errors {
            Errno(errno: errno::Errno) {
                description("errno")
                display("errno: {:?}", errno)
            }
        }
        foreign_links {
            Args(clap::Error);
            Io(io::Error);
            InvalidNum(num::ParseIntError);
            InvalidRegex(regex::Error);
            Uri(hyper::error::UriError) #[cfg(feature="network")];
            Http(hyper::Error) #[cfg(feature="network")];
            Caps(caps::errors::Error) #[cfg(all(target_os="linux", target_arch="x86_64"))];
        }
    }
}
pub use self::error::{Result, Error, ErrorKind};

#[macro_use] mod macros;
pub mod busybox;
#[cfg(feature="network")]
pub mod crypto;
pub mod ctrl;
pub mod ffi;
pub mod shell;

pub use shell::{Shell, Toolbox};
pub use shell::{Command, NativeCommand, ForeignCommand};

/// Arguments passed to builtin commands
pub type Arguments = Vec<String>;
