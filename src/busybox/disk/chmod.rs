use clap::{App, Arg, AppSettings};
use libc::{self, mode_t};
use errno::errno;

use ::{Result, Shell, Arguments};

use std::ffi::CString;


pub fn chmod(sh: &mut Shell, args: Arguments) -> Result<()> {
    let matches = App::new("chmod")
        .setting(AppSettings::DisableVersion)
        .arg(Arg::with_name("mode").required(true))
        .arg(Arg::with_name("path")
            .required(true)
            .multiple(true)
        )
        .get_matches_from_safe(args)?;

    let mode = matches.value_of("mode").unwrap();
    let mode = mode_t::from_str_radix(mode, 8)?;

    for path in matches.values_of("path").unwrap() {
        debug!("chmod: {:?} => {:?}", path, mode);

        let path = CString::new(path).unwrap();
        let ret = unsafe { libc::chmod(path.as_ptr(), mode) };

        if ret != 0 {
            let err = errno();
            shprintln!(sh, "error: {:?}", err);
        }
    }

    Ok(())
}
