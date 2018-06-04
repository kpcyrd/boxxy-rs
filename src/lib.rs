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

#![warn(unused_extern_crates)]
#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

#[cfg(feature="readline")]
extern crate rustyline;
#[macro_use] extern crate log;
extern crate clap;
extern crate libc;
extern crate errno;
extern crate regex;
#[cfg(unix)]
extern crate nix;
extern crate base64;
extern crate bufstream;
#[macro_use] extern crate error_chain;
#[macro_use] extern crate cfg_if;

#[cfg(target_os="linux")]
extern crate caps;

#[cfg(feature="archives")]
extern crate tar;
#[cfg(feature="archives")]
extern crate libflate;

#[cfg(feature="network")]
extern crate rustls;
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
    use std;
    use clap;
    use regex;
    use errno;
    use base64;

    #[cfg(feature="network")]
    use hyper;

    #[cfg(target_os="linux")]
    use caps;

    error_chain! {
        errors {
            Errno(errno: errno::Errno) {
                description("errno")
                display("errno: {:?}", errno)
            }
        }
        foreign_links {
            Args(clap::Error);
            Io(std::io::Error);
            InvalidNum(std::num::ParseIntError);
            InvalidRegex(regex::Error);
            AddrParseError(std::net::AddrParseError);
            Base64Decode(base64::DecodeError);
            Uri(hyper::error::UriError) #[cfg(feature="network")];
            Http(hyper::Error) #[cfg(feature="network")];
            Caps(caps::errors::Error) #[cfg(target_os="linux")];
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

pub use ctrl::Interface;
pub use shell::{Shell, Toolbox};
pub use shell::{Command, NativeCommand, ForeignCommand};

/// Arguments passed to builtin commands
pub type Arguments = Vec<String>;
