use clap::{App, Arg, AppSettings};
use crate::{Shell, Arguments};
use crate::errors::*;
use std::fs::{self, DirEntry};
use std::time::SystemTime;
#[cfg(unix)]
use std::os::unix::fs::MetadataExt;


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

cfg_if::cfg_if! {
    if #[cfg(unix)] {
        #[inline]
        fn decorate(entry: &DirEntry) -> String {
            let meta = entry.metadata().unwrap();
            format!("{} {:5} {:5}  {:14} {:?}",
                perms_to_str(meta.is_dir(), meta.mode()), meta.uid(), meta.gid(),
                meta.modified().ok().map_or_else(|| String::from("-"), since),
                entry.path())
        }
    } else {
        #[inline]
        fn decorate(entry: &DirEntry) -> String {
            let meta = entry.metadata().unwrap();
            format!("{} {:5} {:5}  {:14} {:?}",
                perms_to_str(meta.is_dir(), 0), 0, 0,
                meta.modified().ok().map_or_else(|| String::from("-"), since),
                entry.path())
        }
    }
}

pub fn ls(sh: &mut Shell, args: Arguments) -> Result<()> {
    let matches = App::new("ls")
        .setting(AppSettings::DisableVersion)
        .arg(Arg::with_name("path")
            .multiple(true)
        )
        .arg(Arg::with_name("long").short('l').help("Show more infos"))
        .arg(Arg::with_name("a").short('a').help("Dummy option"))
        .arg(Arg::with_name("h").short('h').help("Dummy option"))
        .get_matches_from_safe(args)?;

    let long = matches.is_present("long");

    let paths = match matches.values_of("path") {
        Some(paths) => paths.into_iter().collect(),
        None => vec!["."],
    };

    for path in paths {
        debug!("ls: {:?}", path);

        match fs::read_dir(path) {
            Ok(entries) => {
                for entry in entries {
                    let entry = entry.unwrap();
                    if long {
                        shprintln!(sh, "{}", decorate(&entry));
                    } else {
                        shprintln!(sh, "{:?}", entry.path());
                    }
                }
            },
            Err(err) => shprintln!(sh, "{:?}: {:?}", path, err),
        }
    }

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permission_formatter() {
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
