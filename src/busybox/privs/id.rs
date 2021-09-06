use crate::{Shell, Arguments};
use crate::errors::*;
use crate::ffi;

cfg_if::cfg_if! {
    if #[cfg(target_os="linux")] {
        pub fn id(sh: &mut Shell, _args: Arguments) -> Result<()> {
            let (ruid, euid, suid) = ffi::getresuid()?;
            let (rgid, egid, sgid) = ffi::getresgid()?;

            let groups = ffi::getgroups()?;

            shprintln!(sh,
                "uid={:?} euid={:?} suid={:?} gid={:?} egid={:?} sgid={:?} groups={:?}",
                ruid,
                euid,
                suid,
                rgid,
                egid,
                sgid,
                groups
            );

            Ok(())
        }
    } else if #[cfg(unix)] {
        pub fn id(sh: &mut Shell, _args: Arguments) -> Result<()> {
            let ruid = ffi::getuid()?;
            let euid = ffi::geteuid()?;

            let rgid = ffi::getgid()?;
            let egid = ffi::getegid()?;

            let groups = ffi::getgroups()?;

            shprintln!(sh,
                "uid={:?} euid={:?} gid={:?} egid={:?} groups={:?}",
                ruid,
                euid,
                rgid,
                egid,
                groups
            );

            Ok(())
        }
    }
}
