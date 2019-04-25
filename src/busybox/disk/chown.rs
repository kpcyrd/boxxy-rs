use clap::{App, Arg, AppSettings};
use libc;
use errno::errno;

use crate::{Result, Shell, Arguments};

use std::ffi::CString;


pub fn chown(sh: &mut Shell, args: Arguments) -> Result<()> {
    let matches = App::new("chown")
        .setting(AppSettings::DisableVersion)
        .arg(Arg::with_name("uid").required(true))
        .arg(Arg::with_name("gid").required(true))
        .arg(Arg::with_name("path")
            .required(true)
            .multiple(true)
        )
        .get_matches_from_safe(args)?;

    let uid = matches.value_of("uid").unwrap().parse()?;
    let gid = matches.value_of("uid").unwrap().parse()?;

    for path in matches.values_of("path").unwrap() {
        debug!("chown: {:?} => {:?}:{:?}", path, uid, gid);

        let path = CString::new(path).unwrap();
        let ret = unsafe { libc::chown(path.as_ptr(), uid, gid) };

        if ret != 0 {
            let err = errno();
            shprintln!(sh, "error: {:?}", err);
        }
    }

    Ok(())
}
