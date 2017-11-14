use clap::{App, Arg, AppSettings};
use libc::{self, gid_t};
use errno::errno;

use ::{Result, Shell, Error, Arguments};
use ffi;

use std::result;


cfg_if! {
    if #[cfg(target_os="linux")] {
        pub fn id(sh: &mut Shell, _args: Arguments) -> Result {
            let (ruid, euid, suid) = ffi::getresuid().unwrap();
            let (rgid, egid, sgid) = ffi::getresgid().unwrap();

            let groups = ffi::getgroups().unwrap();

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
        pub fn id(sh: &mut Shell, _args: Arguments) -> Result {
            let ruid = ffi::getuid().unwrap();
            let euid = ffi::geteuid().unwrap();

            let rgid = ffi::getgid().unwrap();
            let egid = ffi::getegid().unwrap();

            let groups = ffi::getgroups().unwrap();

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


#[cfg(unix)]
pub fn setuid(_sh: &mut Shell, args: Arguments) -> Result {
    let matches = App::new("setuid")
        .setting(AppSettings::DisableVersion)
        .arg(Arg::with_name("uid").required(true))
        .get_matches_from_safe(args)?;

    let uid = matches.value_of("uid").unwrap().parse()?;

    ffi::setuid(uid)
}


#[cfg(unix)]
pub fn seteuid(_sh: &mut Shell, args: Arguments) -> Result {
    let matches = App::new("seteuid")
        .setting(AppSettings::DisableVersion)
        .arg(Arg::with_name("uid").required(true))
        .get_matches_from_safe(args)?;

    let uid = matches.value_of("uid").unwrap().parse()?;

    let ret = unsafe { libc::seteuid(uid) };

    if ret != 0 {
        let err = errno();
        Err(Error::Errno(err))
    } else {
        Ok(())
    }
}


#[cfg(target_os="linux")]
pub fn setreuid(_sh: &mut Shell, args: Arguments) -> Result {
    let matches = App::new("setreuid")
        .setting(AppSettings::DisableVersion)
        .arg(Arg::with_name("ruid").required(true))
        .arg(Arg::with_name("euid").required(true))
        .get_matches_from_safe(args)?;

    let ruid = matches.value_of("ruid").unwrap().parse()?;
    let euid = matches.value_of("euid").unwrap().parse()?;

    let ret = unsafe { libc::setreuid(ruid, euid) };

    if ret != 0 {
        let err = errno();
        Err(Error::Errno(err))
    } else {
        Ok(())
    }
}


#[cfg(target_os="linux")]
pub fn setresuid(_sh: &mut Shell, args: Arguments) -> Result {
    let matches = App::new("setresuid")
        .setting(AppSettings::DisableVersion)
        .arg(Arg::with_name("ruid").required(true))
        .arg(Arg::with_name("euid").required(true))
        .arg(Arg::with_name("suid").required(true))
        .get_matches_from_safe(args)?;

    let ruid = matches.value_of("ruid").unwrap().parse()?;
    let euid = matches.value_of("euid").unwrap().parse()?;
    let suid = matches.value_of("suid").unwrap().parse()?;

    let ret = unsafe { libc::setresuid(ruid, euid, suid) };

    if ret != 0 {
        let err = errno();
        Err(Error::Errno(err))
    } else {
        Ok(())
    }
}


#[cfg(unix)]
pub fn setgid(_sh: &mut Shell, args: Arguments) -> Result {
    let matches = App::new("setgid")
        .setting(AppSettings::DisableVersion)
        .arg(Arg::with_name("gid")
            .required(true)
        )
        .get_matches_from_safe(args)?;

    let uid = matches.value_of("gid").unwrap();
    let uid = uid.parse()?;

    let ret = unsafe { libc::setgid(uid) };

    if ret != 0 {
        let err = errno();
        Err(Error::Errno(err))
    } else {
        Ok(())
    }
}


#[cfg(target_os="linux")]
pub fn setresgid(_sh: &mut Shell, args: Arguments) -> Result {
    let matches = App::new("setresgid")
        .setting(AppSettings::DisableVersion)
        .arg(Arg::with_name("rgid").required(true))
        .arg(Arg::with_name("egid").required(true))
        .arg(Arg::with_name("sgid").required(true))
        .get_matches_from_safe(args)?;

    let rgid = matches.value_of("rgid").unwrap().parse()?;
    let egid = matches.value_of("egid").unwrap().parse()?;
    let sgid = matches.value_of("sgid").unwrap().parse()?;

    let ret = unsafe { libc::setresgid(rgid, egid, sgid) };

    if ret != 0 {
        let err = errno();
        Err(Error::Errno(err))
    } else {
        Ok(())
    }
}


#[cfg(unix)]
pub fn setgroups(_sh: &mut Shell, args: Arguments) -> Result {
    let matches = App::new("setgroups")
        .setting(AppSettings::DisableVersion)
        .arg(Arg::with_name("group")
            .required(true)
            .multiple(true)
        )
        .get_matches_from_safe(args)?;

    let groups: result::Result<Vec<gid_t>, _> = matches.values_of("group").unwrap()
        .map(|x| x.parse())
        .collect();

    let groups = groups.unwrap();

    ffi::setgroups(groups)?;

    Ok(())
}
