//! Builtin commands.

/// Disk related commands
pub mod disk;
#[cfg(unix)]
/// Privilege related commands
pub mod privs;
/// Process related commands
pub mod procs;
/// Network related commands
#[cfg(feature="network")]
pub mod network;

pub use self::disk::*;
#[cfg(unix)]
pub use self::privs::*;
pub use self::procs::*;
#[cfg(feature="network")]
pub use self::network::*;
