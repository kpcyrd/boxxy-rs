use clap::{App, Arg, AppSettings};

use ::{Result, Arguments};

use std::process::Command;


pub fn echo(args: Arguments) -> Result {
    let msg = match args.len() {
        0 | 1 => String::new(),
        _ => {
            let mut msg = args.into_iter().skip(1)
                .fold(String::new(), |a, b| {
                    a + " " + &b
                });
            msg.remove(0);
            msg
        },
    };

    println!("{}", msg);

    Ok(())
}


pub fn exec(mut args: Arguments) -> Result {
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

        Command::new(prog)
            .args(args)
            .spawn()?
            .wait()?;

    }

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[inline]
    fn str_args(args: Vec<&str>) -> Arguments {
        args.into_iter()
            .map(|x| x.to_owned())
            .collect()
    }

    #[test]
    fn test_echo() {
        echo(str_args(vec!["echo", "foo"])).unwrap();
        echo(str_args(vec!["echo", "--", "bar", "asdf"])).unwrap();
        echo(str_args(vec!["echo"])).unwrap();
        echo(str_args(vec!["echo", "-x", "--yz"])).unwrap();
    }
}
