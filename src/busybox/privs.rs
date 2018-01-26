use clap::{App, Arg, AppSettings};
use libc::{self, gid_t};
use errno::errno;

#[cfg(all(target_os="linux", target_arch="x86_64"))]
use caps::{self, Capability, CapSet};

use ::{Result, Shell, ErrorKind, Arguments};
use ffi;

use std::result;
use std::str::FromStr;
use std::collections::HashSet;


cfg_if! {
    if #[cfg(target_os="linux")] {
        pub fn id(sh: &mut Shell, _args: Arguments) -> Result<()> {
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
        pub fn id(sh: &mut Shell, _args: Arguments) -> Result<()> {
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
pub fn setuid(_sh: &mut Shell, args: Arguments) -> Result<()> {
    let matches = App::new("setuid")
        .setting(AppSettings::DisableVersion)
        .arg(Arg::with_name("uid").required(true))
        .get_matches_from_safe(args)?;

    let uid = matches.value_of("uid").unwrap().parse()?;

    ffi::setuid(uid)
}


#[cfg(unix)]
pub fn seteuid(_sh: &mut Shell, args: Arguments) -> Result<()> {
    let matches = App::new("seteuid")
        .setting(AppSettings::DisableVersion)
        .arg(Arg::with_name("uid").required(true))
        .get_matches_from_safe(args)?;

    let uid = matches.value_of("uid").unwrap().parse()?;

    let ret = unsafe { libc::seteuid(uid) };

    if ret != 0 {
        let err = errno();
        Err(ErrorKind::Errno(err).into())
    } else {
        Ok(())
    }
}


#[cfg(target_os="linux")]
pub fn setreuid(_sh: &mut Shell, args: Arguments) -> Result<()> {
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
        Err(ErrorKind::Errno(err).into())
    } else {
        Ok(())
    }
}


#[cfg(target_os="linux")]
pub fn setresuid(_sh: &mut Shell, args: Arguments) -> Result<()> {
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
        Err(ErrorKind::Errno(err).into())
    } else {
        Ok(())
    }
}


#[cfg(unix)]
pub fn setgid(_sh: &mut Shell, args: Arguments) -> Result<()> {
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
        Err(ErrorKind::Errno(err).into())
    } else {
        Ok(())
    }
}


#[cfg(target_os="linux")]
pub fn setresgid(_sh: &mut Shell, args: Arguments) -> Result<()> {
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
        Err(ErrorKind::Errno(err).into())
    } else {
        Ok(())
    }
}


#[cfg(unix)]
pub fn setgroups(_sh: &mut Shell, args: Arguments) -> Result<()> {
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

#[cfg(all(target_os="linux", target_arch="x86_64"))]
pub fn caps(sh: &mut Shell, args: Arguments) -> Result<()> {
    let matches = App::new("caps")
        .setting(AppSettings::DisableVersion)
        .arg(Arg::with_name("effective").short("e"))
        .arg(Arg::with_name("clear").short("c"))
        .arg(Arg::with_name("drop").short("d"))
        .arg(Arg::with_name("add").short("a"))
        .arg(Arg::with_name("set").short("s"))
        .arg(Arg::with_name("capabilities")
            .multiple(true)
        )
        .get_matches_from_safe(args)?;

    let clear = matches.occurrences_of("clear") > 0;
    let drop = matches.occurrences_of("drop") > 0;
    let add = matches.occurrences_of("add") > 0;
    let set = matches.occurrences_of("set") > 0;

    let capabilities = {
        let capabilities: result::Result<HashSet<_>, _> = match matches.values_of("capabilities") {
            Some(caps) => caps.map(|c| Capability::from_str(c)).collect(),
            None => Ok(HashSet::new()),
        };

        capabilities.unwrap()
    };

    let capset = match matches.occurrences_of("effective") > 0 {
        true  => CapSet::Effective,
        false => CapSet::Permitted,
    };

    if clear {
        info!("caps(clear)");
        caps::clear(None, capset).unwrap();
    } else if drop {
        for cap in capabilities {
            info!("caps(drop): {:?}", cap);
            caps::drop(None, capset, cap).unwrap();
        }
    } else if add {
        for cap in capabilities {
            info!("caps(raise): {:?}", cap);
            caps::raise(None, capset, cap).unwrap();
        }
    } else if set {
        info!("caps(set): {:?}", capabilities);
        caps::set(None, capset, capabilities).unwrap();
    } else {
        let caps = caps::read(None, capset).unwrap();
        shprintln!(sh, "{:?}", caps);
    }

    Ok(())
}
