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

#[macro_use] extern crate log;
#[macro_use] extern crate error_chain;
#[macro_use] extern crate cfg_if;

mod error {
    use std;
    use clap;
    use regex;
    use errno;
    use base64;

    #[cfg(feature="network")]
    use hyper;

    #[cfg(feature="network")]
    use http;

    #[cfg(target_os="linux")]
    use caps;

    #[cfg(target_os="openbsd")]
    use pledge;

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
            Uri(http::uri::InvalidUri) #[cfg(feature="network")];
            Http(hyper::Error) #[cfg(feature="network")];
            Caps(caps::errors::Error) #[cfg(target_os="linux")];
            Pledge(pledge::Error) #[cfg(target_os="openbsd")];
        }
    }
}
pub use self::error::{Result, Error, ErrorKind};

#[macro_use] mod macros;
pub mod busybox;
#[cfg(feature="readline")]
pub mod completer;
#[cfg(feature="network")]
pub mod crypto;
pub mod ctrl;
pub mod ffi;
pub mod shell;

pub use crate::ctrl::Interface;
pub use crate::shell::{Shell, Toolbox};
pub use crate::shell::{Command, NativeCommand, ForeignCommand};

/// Arguments passed to builtin commands
pub type Arguments = Vec<String>;
