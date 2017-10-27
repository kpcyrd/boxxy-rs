use clap::{App, Arg, AppSettings};
use libc::{self, gid_t};
use errno::errno;

use ::{Result, Error, Arguments};
use ffi;

use std::result;


pub fn id(_args: Arguments) -> Result {
    let (ruid, euid, suid) = ffi::getresuid().unwrap();
    let (rgid, egid, sgid) = ffi::getresgid().unwrap();

    let groups = ffi::getgroups().unwrap();

    println!(
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


pub fn setuid(args: Arguments) -> Result {
    let matches = App::new("setuid")
        .setting(AppSettings::DisableVersion)
        .arg(Arg::with_name("uid").required(true))
        .get_matches_from_safe(args)?;

    let uid = matches.value_of("uid").unwrap().parse()?;

    let ret = unsafe { libc::setuid(uid) };

    if ret != 0 {
        let err = errno();
        Err(Error::Errno(err))
    } else {
        Ok(())
    }
}


pub fn seteuid(args: Arguments) -> Result {
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


pub fn setreuid(args: Arguments) -> Result {
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


pub fn setresuid(args: Arguments) -> Result {
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


pub fn setgid(args: Arguments) -> Result {
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


pub fn setresgid(args: Arguments) -> Result {
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


pub fn setgroups(args: Arguments) -> Result {
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
