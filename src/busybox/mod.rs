//! Builtin commands.

/// Disk related commands
pub mod disk;
/// Privilege related commands
pub mod privs;
/// Process related commands
pub mod procs;

pub use self::disk::*;
pub use self::privs::*;
pub use self::procs::*;
