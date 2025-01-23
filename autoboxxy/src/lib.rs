extern crate boxxy;

use boxxy::{Interface, Shell, Toolbox};
use std::env;
use std::fs::File;

/* Rust doesn't directly expose __attribute__((constructor)), but this
 * is how GNU implements it.
 * Props to https://github.com/geofft/redhook */
#[link_section = ".init_array"]
pub static INITIALIZE_CTOR: extern "C" fn() = ::initialize;

/*
// alternative hook
#[link_section=".ctors"]
#[no_mangle]
pub static CALL_BOXXY: extern fn() = ::initialize;
*/

extern "C" fn initialize() {
    env::remove_var("LD_PRELOAD");

    if let Ok(cmd) = env::var("AUTOBOXXY") {
        let toolbox = Toolbox::new();
        let mut shell = Shell::new(toolbox);
        if let Ok(target) = env::var("AUTOBOXXY_OUTPUT") {
            let output = File::create(target).unwrap();
            shell.hotswap(Interface::file(None, Some(output)));
        }
        shell.exec_once(&cmd);
        std::process::exit(0);
    }
}
