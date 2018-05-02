extern crate boxxy;

use std::env;
use boxxy::{Shell, Toolbox};

/* Rust doesn't directly expose __attribute__((constructor)), but this
 * is how GNU implements it.
 * Props to https://github.com/geofft/redhook */
#[link_section=".init_array"]
pub static INITIALIZE_CTOR: extern fn() = ::initialize;

/*
// alternative hook
#[link_section=".ctors"]
#[no_mangle]
pub static CALL_BOXXY: extern fn() = ::initialize;
*/

extern fn initialize() {
    env::remove_var("LD_PRELOAD");

    if let Ok(cmd) = env::var("AUTOBOXXY") {
        let toolbox = Toolbox::new();
        let mut shell = Shell::new(toolbox);
        shell.exec_once(&cmd);
        std::process::exit(0);
    }
}
