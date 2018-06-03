use clap::{App, Arg, AppSettings, ArgGroup};
use libc::{self, gid_t};
use errno::errno;

#[cfg(target_os="linux")]
use caps::{self, Capability, CapSet};

use ::{Result, Shell, ErrorKind, Arguments};
use ffi;

use std::result;
#[cfg(target_os="linux")]
use std::str::FromStr;
#[cfg(target_os="linux")]
use std::collections::HashSet;


cfg_if! {
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


#[cfg(unix)]
pub fn setuid(_sh: &mut Shell, args: Arguments) -> Result<()> {
    let matches = App::new("setuid")
        .setting(AppSettings::DisableVersion)
        .about("Call setuid(2)")
        .arg(Arg::with_name("uid").required(true))
        .get_matches_from_safe(args)?;

    let uid = matches.value_of("uid").unwrap().parse()?;

    ffi::setuid(uid)
}


#[cfg(unix)]
pub fn seteuid(_sh: &mut Shell, args: Arguments) -> Result<()> {
    let matches = App::new("seteuid")
        .setting(AppSettings::DisableVersion)
        .about("Call seteuid(2)")
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
        .about("Call setreuid(2)")
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
        .about("Call setresuid(2)")
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
        .about("Call setgid(2)")
        .arg(Arg::with_name("gid")
            .required(true)
        )
        .get_matches_from_safe(args)?;

    let uid = matches.value_of("gid").unwrap().parse()?;

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
        .about("Call setresgid(2)")
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
        .about("Call setgroups(2)")
        .arg(Arg::with_name("group")
            .required(true)
            .multiple(true)
            .help("The groups that should be set")
        )
        .get_matches_from_safe(args)?;

    let groups = matches.values_of("group").unwrap()
        .map(|x| x.parse())
        .collect::<result::Result<Vec<gid_t>, _>>()?;

    ffi::setgroups(&groups)?;

    Ok(())
}

#[cfg(target_os="linux")]
pub fn caps(sh: &mut Shell, args: Arguments) -> Result<()> {
    let matches = App::new("caps")
        .setting(AppSettings::DisableVersion)
        .group(ArgGroup::with_name("capset")
            .args(&["effective", "bounding", "inheritable", "ambient"])
        )
        .arg(Arg::with_name("effective")
                    .short("e")
                    .long("effective")
                    .help("Operate on the effective capset instead of the permitted capset"))
        .arg(Arg::with_name("bounding")
                    .short("b")
                    .long("bounding")
                    .help("Operate on the bounding capset instead of the permitted capset"))
        .arg(Arg::with_name("inheritable")
                    .short("i")
                    .long("inheritable")
                    .help("Operate on the inheritable capset instead of the permitted capset"))
        .arg(Arg::with_name("ambient")
                    .short("a")
                    .long("ambient")
                    .help("Operate on the ambient capset instead of the permitted capset"))
        .arg(Arg::with_name("clear")
                    .short("c")
                    .long("clear")
                    .help("Clear all capabilities"))
        .arg(Arg::with_name("drop")
                    .short("d")
                    .long("drop")
                    .help("Drop specific capabilities"))
        .arg(Arg::with_name("raise")
                    .short("r")
                    .long("raise")
                    .help("Add capabilities to capability set"))
        .arg(Arg::with_name("set")
                    .short("s")
                    .long("set")
                    .help("Set the capability set"))
        .arg(Arg::with_name("capabilities")
            .multiple(true)
        )
        .get_matches_from_safe(args)?;

    let clear = matches.occurrences_of("clear") > 0;
    let drop = matches.occurrences_of("drop") > 0;
    let raise = matches.occurrences_of("raise") > 0;
    let set = matches.occurrences_of("set") > 0;

    let capabilities = match matches.values_of("capabilities") {
        Some(caps) => caps.map(|c| Capability::from_str(c))
                          .collect::<result::Result<_, _>>()?,
        None => HashSet::new(),
    };

    let capset = if matches.is_present("effective") {
        CapSet::Effective
    } else if matches.is_present("bounding") {
        CapSet::Bounding
    } else if matches.is_present("inheritable") {
        CapSet::Inheritable
    } else if matches.is_present("ambient") {
        CapSet::Ambient
    } else {
        CapSet::Permitted
    };

    if clear {
        info!("caps(clear)");
        caps::clear(None, capset)?;
    } else if drop {
        for cap in capabilities {
            info!("caps(drop): {:?}", cap);
            caps::drop(None, capset, cap)?;
        }
    } else if raise {
        for cap in capabilities {
            info!("caps(raise): {:?}", cap);
            caps::raise(None, capset, cap)?;
        }
    } else if set {
        info!("caps(set): {:?}", capabilities);
        caps::set(None, capset, capabilities)?;
    } else {
        let caps = caps::read(None, capset)?;
        shprintln!(sh, "{:?}", caps);
    }

    Ok(())
}
