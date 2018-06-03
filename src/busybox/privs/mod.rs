#[cfg(unix)]
mod id;
#[cfg(unix)]
pub use self::id::id;

#[cfg(unix)]
mod setuid;
#[cfg(unix)]
pub use self::setuid::setuid;

#[cfg(unix)]
mod seteuid;
#[cfg(unix)]
pub use self::seteuid::seteuid;

#[cfg(target_os="linux")]
mod setreuid;
#[cfg(target_os="linux")]
pub use self::setreuid::setreuid;

#[cfg(target_os="linux")]
mod setresuid;
#[cfg(target_os="linux")]
pub use self::setresuid::setresuid;

#[cfg(unix)]
mod setgid;
#[cfg(unix)]
pub use self::setgid::setgid;

#[cfg(target_os="linux")]
mod setresgid;
#[cfg(target_os="linux")]
pub use self::setresgid::setresgid;

#[cfg(unix)]
mod setgroups;
#[cfg(unix)]
pub use self::setgroups::setgroups;

#[cfg(target_os="linux")]
mod caps;
#[cfg(target_os="linux")]
pub use self::caps::caps;
