use clap::{App, Arg, AppSettings};
use libc::{self, mode_t};
use errno::errno;
use regex::Regex;
use nix;

use ::{Result, Error, Arguments};

use std::fs;
use std::env;
use std::ffi::CString;
use std::io::BufReader;
use std::io::prelude::*;
use std::time::SystemTime;
use std::os::unix::fs::MetadataExt;


pub fn cat(args: Arguments) -> Result {
    let matches = App::new("cat")
        .setting(AppSettings::DisableVersion)
        .arg(Arg::with_name("path")
            .multiple(true)
            .required(true)
        )
        .get_matches_from_safe(args)?;

    for path in matches.values_of("path").unwrap() {
        debug!("cat: {:?}", path);

        match fs::File::open(path) {
            Ok(file) => {
                let reader = BufReader::new(file);
                for line in reader.lines() {
                    println!("{:?}", line);
                }
            },
            Err(err) => {
                println!("error: {:?}", err);
            },
        };
    }

    Ok(())
}


pub fn cd(args: Arguments) -> Result {
    let matches = App::new("cd")
        .setting(AppSettings::DisableVersion)
        .arg(Arg::with_name("path")
            .required(true)
        )
        .get_matches_from_safe(args)?;

    let path = matches.value_of("path").unwrap();

    env::set_current_dir(&path)?;

    Ok(())
}


pub fn chmod(args: Arguments) -> Result {
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
            println!("error: {:?}", Error::Errno(err));
        }
    }

    Ok(())
}


pub fn chown(args: Arguments) -> Result {
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
            println!("error: {:?}", Error::Errno(err));
        }
    }

    Ok(())
}


pub fn chroot(args: Arguments) -> Result {
    let matches = App::new("chroot")
        .setting(AppSettings::DisableVersion)
        .arg(Arg::with_name("path")
            .required(true)
        )
        .get_matches_from_safe(args)?;

    let path = matches.value_of("path").unwrap();
    let path = CString::new(path).unwrap();

    let ret = unsafe { libc::chroot(path.as_ptr()) };

    if ret != 0 {
        let err = errno();
        Err(Error::Errno(err))
    } else {
        Ok(())
    }
}


pub fn grep(args: Arguments) -> Result {

    // TODO: -i
    // TODO: -r
    // TODO: -n

    let matches = App::new("grep")
        .setting(AppSettings::DisableVersion)
        .arg(Arg::with_name("pattern")
            .required(true)
        )
        .arg(Arg::with_name("path")
            .multiple(true)
            .required(true)
        )
        .get_matches_from_safe(args)?;

    let pattern = matches.value_of("pattern").unwrap();
    let pattern = Regex::new(pattern)?;

    for path in matches.values_of("path").unwrap() {
        debug!("grep: {:?}", path);

        match fs::File::open(path) {
            Ok(file) => {
                let reader = BufReader::new(file);
                for line in reader.lines() {
                    let line = line?;

                    if pattern.is_match(&line) {
                        println!("{:?}", line);
                    }
                }
            },
            Err(err) => {
                println!("error: {:?}", err);
            },
        };
    }

    Ok(())
}


fn perms_to_str(is_dir: bool, permissions: u32) -> String {
    let mut out: String = String::with_capacity(10);

    out.push(if is_dir { 'd' } else { '-' });

    out.push(if permissions & 0o400 > 0 { 'r' } else { '-' });
    out.push(if permissions & 0o200 > 0 { 'w' } else { '-' });
    out.push(if permissions & 0o100 > 0 { 'x' } else { '-' });

    out.push(if permissions &  0o40 > 0 { 'r' } else { '-' });
    out.push(if permissions &  0o20 > 0 { 'w' } else { '-' });
    out.push(if permissions &  0o10 > 0 { 'x' } else { '-' });

    out.push(if permissions &   0o4 > 0 { 'r' } else { '-' });
    out.push(if permissions &   0o2 > 0 { 'w' } else { '-' });
    out.push(if permissions &   0o1 > 0 { 'x' } else { '-' });

    out
}


fn since(time: SystemTime) -> String {
    let now = SystemTime::now();

    if now < time {
        String::from("future?!")
    } else {
        let duration = now.duration_since(time).unwrap().as_secs();

        let mut duration = (duration, "sec");

        for &(x, unit) in &[
                            (60, "min"),
                            (60, "hour"),
                            (24, "day"),
                            (31, "month"),
                            (12, "year"),
                           ] {
            if duration.0 > x {
                duration = (duration.0 / x, unit);
            } else {
                break;
            }
        }

        if duration.0 == 1 {
            format!("{:?} {} ago", duration.0, duration.1)
        } else {
            format!("{:?} {}s ago", duration.0, duration.1)
        }
    }
}


pub fn ls(args: Arguments) -> Result {
    let matches = App::new("ls")
        .setting(AppSettings::DisableVersion)
        .arg(Arg::with_name("path")
            .multiple(true)
        )
        .arg(Arg::with_name("long").short("l"))
        .arg(Arg::with_name("a").short("a"))
        .get_matches_from_safe(args)?;

    let long = matches.occurrences_of("long") > 0;

    let paths = match matches.values_of("path") {
        Some(paths) => paths.into_iter().map(|x| x).collect(),
        None => vec!["."],
    };

    for path in paths {
        debug!("ls: {:?}", path);

        match fs::read_dir(&path) {
            Ok(entries) => {
                for entry in entries {
                    let entry = entry.unwrap();
                    if long {
                        let meta = entry.metadata().unwrap();
                        println!("{} {:5} {:5}  {:14} {:?}",
                            perms_to_str(meta.is_dir(), meta.mode()), meta.uid(), meta.gid(),
                            meta.modified().map(|x| since(x)).unwrap_or(String::from("-")),
                            entry.path());
                    } else {
                        println!("{:?}", entry.path());
                    }
                }
            },
            Err(err) => println!("{:?}: {:?}", path, err),
        }
    }

    Ok(())
}


pub fn mkdir(args: Arguments) -> Result {
    let matches = App::new("mount")
        .setting(AppSettings::DisableVersion)
        .arg(Arg::with_name("directory")
            .required(true)
        )
        .arg(Arg::with_name("parents")
            .short("p")
            .long("parents")
        )
        .get_matches_from_safe(args)?;

    let directory = matches.value_of("directory").unwrap();
    let parents = matches.occurrences_of("parents") > 0;

    if parents {
        fs::create_dir_all(directory)?;
    } else {
        fs::create_dir(directory)?;
    }

    Ok(())
}


pub fn mount(args: Arguments) -> Result {
    let matches = App::new("mount")
        .setting(AppSettings::DisableVersion)
        .arg(Arg::with_name("src")
            .required(true)
        )
        .arg(Arg::with_name("dest"))
        .arg(Arg::with_name("fstype")
            .short("t")
            .takes_value(true)
        )
        .get_matches_from_safe(args)?;

    let (src, dest) = match matches.value_of("dest") {
        Some(dest) => {
            let src = matches.value_of("src");
            (src, dest)
        },
        None => {
            let src = None;
            let dest = matches.value_of("src").unwrap();
            (src, dest)
        },
    };

    let fstype = matches.value_of("fstype");
    let flags = nix::mount::MsFlags::empty();

    let data: Option<&'static [u8]> = None;

    if let Err(err) = nix::mount::mount(src, dest, fstype, flags, data) {
        println!("error: mount: {:?}", err);
    }

    Ok(())
}


pub fn pwd(_args: Arguments) -> Result {
    let path = env::current_dir().unwrap();
    let path = path.to_str().unwrap().to_owned();
    println!("{:?}", path);
    Ok(())
}


pub fn rm(args: Arguments) -> Result {
    let matches = App::new("rm")
        .setting(AppSettings::DisableVersion)
        .arg(Arg::with_name("path")
            .required(true)
            .multiple(true)
        )
        .arg(Arg::with_name("r").short("r"))
        .arg(Arg::with_name("f").short("f"))
        .get_matches_from_safe(args)?;

    let recursive = matches.occurrences_of("r") > 0;

    for path in matches.values_of("path").unwrap() {
        debug!("rm: {:?}", path);

        let result = match recursive {
            true  => fs::remove_dir_all(path),
            false => fs::remove_file(path),
        };

        match result {
            Ok(_) => (),
            Err(err) => println!("rm: {:?}: {:?}", path, err),
        }
    }

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_comment() {
        assert_eq!("drwxrwxrwx", perms_to_str(true, 0o777));
        assert_eq!("-rwxrwxrwx", perms_to_str(false, 0o777));
        assert_eq!("drwxr-xr-x", perms_to_str(true, 0o755));
        assert_eq!("-rw-r--r--", perms_to_str(false, 0o644));
        assert_eq!("drwx------", perms_to_str(true, 0o700));
        assert_eq!("-rwxr-xr-x", perms_to_str(false, 0o4755));
        assert_eq!("----------", perms_to_str(false, 0o000));
        assert_eq!("----------", perms_to_str(false, 0o4000));
    }
}
