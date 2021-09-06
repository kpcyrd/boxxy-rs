use crate::errors::*;
use crate::shell;
use libc::{uid_t, gid_t};
use std::process;

pub fn getuid() -> Result<uid_t> {
    let uid = unsafe { libc::getuid() };
    Ok(uid)
}

pub fn geteuid() -> Result<uid_t> {
    let euid = unsafe { libc::geteuid() };
    Ok(euid)
}

pub fn setuid(uid: uid_t) -> Result<()> {
    let ret = unsafe { libc::setuid(uid) };

    if ret != 0 {
        Err(errno())
    } else {
        Ok(())
    }
}


pub fn getgid() -> Result<uid_t> {
    let gid = unsafe { libc::getgid() };
    Ok(gid)
}


pub fn getegid() -> Result<uid_t> {
    let egid = unsafe { libc::getegid() };
    Ok(egid)
}


/// Get the supplemental groups.
///
/// ```
/// let groups = boxxy::ffi::getgroups().unwrap();
/// println!("groups={:?}", groups);
/// ```
pub fn getgroups() -> Result<Vec<gid_t>> {
    let size = 128;
    let mut gids: Vec<gid_t> = Vec::with_capacity(size as usize);

    let ret = unsafe { libc::getgroups(size, gids.as_mut_ptr()) };

    if ret < 0 {
        Err(errno())
    } else {
        let groups = (0..ret)
            .map(|i| unsafe { gids.get_unchecked(i as usize) }.to_owned())
            .collect();
        Ok(groups)
    }
}

#[derive(Debug)]
pub enum Fork {
    Parent(i32),
    Child,
}

pub fn fork() -> Result<Fork> {
    let ret = unsafe { libc::fork() };
    match ret {
        ret if ret < 0 => Err(errno()),
        ret if ret > 0 => Ok(Fork::Parent(ret)),
        _ => Ok(Fork::Child),
    }
}

pub fn waitpid(pid: i32) {
    unsafe { libc::waitpid(pid, ::std::ptr::null_mut(), 0) };
}


pub fn daemonize(mut shell: shell::Shell, func: shell::Command, args: Vec<String>) -> Result<()> {
    match fork()? {
        Fork::Parent(pid) => {
            unsafe { libc::waitpid(pid, ::std::ptr::null_mut(), 0) };
        },
        Fork::Child => {
            let ret = unsafe { libc::setsid() };
            if ret < 0 {
                println!("{:#}", errno());
                process::exit(1);
            }

            if let Fork::Parent(_) = fork()? {
                process::exit(0);
            }

            process::exit(match func.run(&mut shell, args) {
                Ok(_) => 0,
                Err(_) => 1,
            });
        }
    }
    Ok(())
}
