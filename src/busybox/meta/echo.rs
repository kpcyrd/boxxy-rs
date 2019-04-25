use crate::{Result, Shell, Arguments};

pub fn echo(sh: &mut Shell, args: Arguments) -> Result<()> {
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

    shprintln!(sh, "{}", msg);

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::ctrl;
    use crate::Toolbox;

    #[inline]
    fn str_args(args: Vec<&str>) -> Arguments {
        args.into_iter()
            .map(|x| x.to_owned())
            .collect()
    }

    #[test]
    fn test_echo() {
        let mut sh = Shell::new(Toolbox::empty());
        sh.hotswap(ctrl::Interface::dummy());

        echo(&mut sh, str_args(vec!["echo", "foo"])).unwrap();
        echo(&mut sh, str_args(vec!["echo", "--", "bar", "asdf"])).unwrap();
        echo(&mut sh, str_args(vec!["echo"])).unwrap();
        echo(&mut sh, str_args(vec!["echo", "-x", "--yz"])).unwrap();
    }
}
