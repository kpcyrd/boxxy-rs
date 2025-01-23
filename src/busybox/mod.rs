//! Builtin commands.

macro_rules! import_cmd {
    ($x:ident) => {
        mod $x;
        pub use self::$x::$x;
    };
}

/// Disk related commands
pub mod disk;
/// Meta commands
pub mod meta;
/// Network related commands
#[cfg(feature = "network")]
pub mod network;
#[cfg(unix)]
/// Privilege related commands
pub mod privs;
/// Process related commands
pub mod procs;

pub use self::disk::*;
pub use self::meta::*;
#[cfg(feature = "network")]
pub use self::network::*;
#[cfg(unix)]
pub use self::privs::*;
pub use self::procs::*;
