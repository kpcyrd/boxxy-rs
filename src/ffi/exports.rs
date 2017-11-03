use libc;

use ::{Shell, Toolbox};
use ffi::ForeignCommand;
use std::ffi::CStr;


/// Crate a shell, returns a pointer.
#[no_mangle]
pub extern fn boxxy_init() -> *mut Shell {
    let shell = Shell::new(Toolbox::new());
    Box::into_raw(Box::new(shell))
}

/// Drop into a shell with default config.
#[no_mangle]
pub extern fn boxxy_run() {
    Shell::new(Toolbox::new()).run();
}

/// Extend the shell struct with additional commands.
#[no_mangle]
pub extern fn boxxy_with(target: *mut Shell, name: *const libc::c_char, ptr: ForeignCommand) {
    let name = unsafe {
        let bytes = CStr::from_ptr(name).to_bytes();
        String::from_utf8(bytes.to_vec()).ok().expect("Invalid UTF8 string").to_string()
    };

    debug!("registering: {:?} -> {:?}", name, ptr);
    unsafe { (&mut *target) }.insert(name, ptr.into());
}

/// Execute a single command.
#[no_mangle]
pub extern fn boxxy_exec_once_at(target: *mut Shell, cmd: *const libc::c_char) -> i32 {
    let cmd = unsafe {
        let bytes = CStr::from_ptr(cmd).to_bytes();
        String::from_utf8(bytes.to_vec()).ok().expect("Invalid UTF8 string").to_string()
    };

    unsafe { (&mut *target) }.exec_once(&cmd);

    0
}

/// Start shell at specific pointer.
#[no_mangle]
pub extern fn boxxy_run_at(target: *mut Shell) {
    unsafe { (&mut *target) }.run()
}

/// Free memory.
#[no_mangle]
pub unsafe extern fn boxxy_free(target: *mut Shell) {
    if !target.is_null() {
        Box::from_raw(target);
    }
}
