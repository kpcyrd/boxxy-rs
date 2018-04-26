extern crate boxxy;
extern crate env_logger;
extern crate libc;
extern crate errno;

use std::env;
use std::ffi::CString;

#[cfg(unix)]
fn chroot(path: &str) -> Result<(), errno::Errno> {
    let path = CString::new(path).unwrap();
    let ret = unsafe { libc::chroot(path.as_ptr()) };

    if ret == 0 {
        Ok(())
    } else {
        Err(errno::errno())
    }
}

#[cfg(unix)]
fn getuid() -> libc::uid_t {
    unsafe { libc::getuid() }
}

#[cfg(unix)]
fn main() {
    if getuid() != 0 {
        println!("Error: this challenge needs root to set up");
        println!("\tcargo build --examples && sudo target/debug/examples/vuln-chroot");
        return;
    }

    let jail = "jails/empty/";
    println!("[*] creating jail at {:?}", jail);
    std::fs::create_dir_all(&jail).unwrap();

    println!("[*] starting chroot...");
    chroot(&jail).unwrap();

    env::set_current_dir("/").unwrap();

    println!("[+] jail is active, can you escape it?");

    boxxy::Shell::new(boxxy::Toolbox::new()).run()
}

#[cfg(not(unix))]
fn main() {
    panic!("unsupported platform");
}
