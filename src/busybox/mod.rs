//! Builtin commands.

/// Disk related commands
pub mod disk;
#[cfg(target_os="linux")]
/// Kernel related commands
pub mod kernel;
#[cfg(unix)]
/// Privilege related commands
pub mod privs;
/// Process related commands
pub mod procs;
/// Meta commands
pub mod meta;
/// Network related commands
#[cfg(feature="network")]
pub mod network;

pub use self::disk::*;
#[cfg(target_os="linux")]
pub use self::kernel::*;
#[cfg(unix)]
pub use self::privs::*;
pub use self::procs::*;
pub use self::meta::*;
#[cfg(feature="network")]
pub use self::network::*;
