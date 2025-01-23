#![allow(clippy::missing_safety_doc)]

use crate::errors::*;
use crate::ffi::ForeignCommand;
use crate::{Shell, Toolbox};
use std::ffi::CStr;

/// Crate a shell, returns a pointer.
#[no_mangle]
pub extern "C" fn boxxy_init() -> *mut Shell {
    let shell = Shell::new(Toolbox::new());
    Box::into_raw(Box::new(shell))
}

/// Drop into a shell with default config.
#[no_mangle]
pub extern "C" fn boxxy_run() {
    Shell::new(Toolbox::new()).run();
}

/// Extend the shell struct with additional commands.
#[no_mangle]
pub unsafe extern "C" fn boxxy_with(
    target: *mut Shell,
    name: *const libc::c_char,
    ptr: ForeignCommand,
) {
    let bytes = CStr::from_ptr(name).to_bytes();
    let name = String::from_utf8(bytes.to_vec()).expect("Invalid UTF8 string");

    debug!("registering: {:?} -> {:?}", name, ptr);
    (*target).insert(name, ptr.into());
}

/// Execute a single command.
#[no_mangle]
pub unsafe extern "C" fn boxxy_exec_once_at(target: *mut Shell, cmd: *const libc::c_char) -> i32 {
    let bytes = CStr::from_ptr(cmd).to_bytes();
    let cmd = String::from_utf8(bytes.to_vec()).expect("Invalid UTF8 string");

    (*target).exec_once(&cmd);

    0
}

/// Start shell at specific pointer.
#[no_mangle]
pub unsafe extern "C" fn boxxy_run_at(target: *mut Shell) {
    (*target).run()
}

/// Free memory.
#[no_mangle]
pub unsafe extern "C" fn boxxy_free(target: *mut Shell) {
    if !target.is_null() {
        drop(Box::from_raw(target));
    }
}
