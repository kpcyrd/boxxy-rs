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

#[macro_use] mod macros;
pub mod busybox;
#[cfg(feature="readline")]
pub mod completer;
#[cfg(feature="network")]
pub mod crypto;
pub mod ctrl;
pub mod errors;
pub mod ffi;
pub mod shell;

pub use crate::ctrl::Interface;
pub use errors::{Result, Error};
pub use crate::shell::{Shell, Toolbox};
pub use crate::shell::{Command, NativeCommand, ForeignCommand};

/// Arguments passed to builtin commands
pub type Arguments = Vec<String>;
