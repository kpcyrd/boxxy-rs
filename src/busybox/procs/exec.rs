use clap::{App, Arg, AppSettings};
#[cfg(unix)]
use libc;

use crate::{Result, Shell, Arguments};

use std::process::Command;
#[cfg(unix)]
use std::process::Stdio;
#[cfg(unix)]
use std::os::unix::io::FromRawFd;


pub fn exec(sh: &mut Shell, mut args: Arguments) -> Result<()> {
    if args.len() < 2 {
        // triggers an usage errror
        let _ = App::new("exec")
            .setting(AppSettings::DisableVersion)
            .arg(Arg::with_name("prog")
                .required(true)
            )
            .arg(Arg::with_name("args")
                .multiple(true)
            )
            .get_matches_from_safe(args)?;
    } else {
        let _ = args.remove(0);

        let prog = args.remove(0);

        let mut child = Command::new(prog);
        child.args(args);

        #[cfg(unix)]
        {
            if let Some((stdin, stdout, stderr)) = sh.pipe() {
                // if stdio needs redirection
                // this is only supported on unix
                unsafe {
                    child.stdin(Stdio::from_raw_fd(libc::dup(stdin)))
                        .stdout(Stdio::from_raw_fd(libc::dup(stdout)))
                        .stderr(Stdio::from_raw_fd(libc::dup(stderr)));
                }
            }
        }

        child.spawn()?
            .wait()?;
    }

    Ok(())
}
