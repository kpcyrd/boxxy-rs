use busybox;
use clap;
use rustyline::{self, Editor};
use rustyline::completion::Completer;

use Error;
use std::collections::HashMap;

fn parse(line: &str) -> Vec<String> {
    let mut cmd = Vec::new();

    let mut token = String::new();

    let mut escape = false;
    for x in line.chars() {
        if escape {
            token.push(x);
            escape = false;
            continue;
        }

        match x {
            ' ' | '\n' => {
                if token.len() > 0 {
                    cmd.push(token);
                    token = String::new();
                }
            },
            '\\' => {
                escape = true;
            },
            x => {
                token.push(x);
            },
        }
    }

    if token.len() > 0 {
            cmd.push(token);
    }

    cmd
}


struct CmdCompleter {
    commands: Vec<String>,
}

impl CmdCompleter {
    fn new(toolbox: &Toolbox) -> CmdCompleter {
        CmdCompleter {
            commands: toolbox.keys(),
        }
    }
}

impl Completer for CmdCompleter {
    fn complete(&self, line: &str, pos: usize) -> rustyline::Result<(usize, Vec<String>)> {
        if line.contains(" ") || line.len() != pos {
            return Ok((0, vec![]));
        }

        let results: Vec<String> = self.commands.iter()
            .filter(|x| x.starts_with(line))
            .map(|x| x.clone() + " ")
            .collect();

        Ok((0, results))
    }
}


pub struct Toolbox {
    commands: HashMap<String, Command>,
}

impl Toolbox {
    #[inline]
    pub fn empty() -> Toolbox {
        Toolbox {
            commands: HashMap::new(),
        }
    }

    #[inline]
    pub fn new() -> Toolbox {
        let mut toolbox = Toolbox::empty();
        toolbox.insert_many(vec![
            ("cat"          , busybox::cat),
            ("cd"           , busybox::cd),
            ("chmod"        , busybox::chmod),
            ("chown"        , busybox::chown),
            ("chroot"       , busybox::chroot),
            ("echo"         , busybox::echo),
            ("exec"         , busybox::exec),
            ("grep"         , busybox::grep),
            ("ls"           , busybox::ls),
            ("mkdir"        , busybox::mkdir),
            ("mount"        , busybox::mount),
            ("id"           , busybox::id),
            ("pwd"          , busybox::pwd),
            ("rm"           , busybox::rm),
            ("setgroups"    , busybox::setgroups),
            ("setgid"       , busybox::setgid),
            ("setresgid"    , busybox::setresgid),
            ("setuid"       , busybox::setuid),
            ("seteuid"      , busybox::seteuid),
            ("setreuid"     , busybox::setreuid),
            ("setresuid"    , busybox::setresuid),
        ]);
        toolbox
    }

    #[inline]
    pub fn get(&self, key: &str) -> Option<&Command> {
        self.commands.get(key)
    }

    #[inline]
    pub fn keys(&self) -> Vec<String> {
        self.commands
            .keys()
            .map(|x| x.to_owned())
            .collect()
    }

    #[inline]
    pub fn insert(&mut self, key: &str, func: Command) {
        self.commands.insert(key.into(), func);
    }

    #[inline]
    pub fn with(mut self, commands: Vec<(&str, Command)>) -> Toolbox {
        self.insert_many(commands);
        self
    }

    #[inline]
    pub fn insert_many(&mut self, commands: Vec<(&str, Command)>) {
        for (key, func) in commands {
            self.insert(key, func);
        }
    }
}


type Command = fn(Vec<String>) -> Result<(), Error>;


pub struct Shell {
    rl: Editor<CmdCompleter>,
    toolbox: Toolbox,
}

impl Shell {
    pub fn new(toolbox: Toolbox) -> Shell {
        let c = CmdCompleter::new(&toolbox);

        let mut rl = Editor::new();
        rl.set_completer(Some(c));

        Shell {
            rl,
            toolbox,
        }
    }

    fn process(&self, prog: String, args: Vec<String>) {
        let result = match self.toolbox.get(&prog) {
            Some(func) => func(args),
            None => Err(Error::Args(clap::Error {
                    message: String::from("\u{1b}[1;31merror:\u{1b}[0m unknown command"),
                    kind: clap::ErrorKind::MissingRequiredArgument,
                    info: None,
            })),
        };

        if let Err(err) = result {
            match err {
                Error::Args(err)         => println!("{}", err.message),
                Error::Io(err)           => println!("error: {:?}", err),
                Error::Errno(err)        => println!("error: {:?}", err),
                Error::InvalidNum(err)   => println!("error: {:?}", err),
                Error::InvalidRegex(err) => println!("error: {:?}", err),
            }
        }
    }

    fn prompt(&mut self) -> Result<String, rustyline::error::ReadlineError> {
        self.rl.readline(" [%]> ")
    }

    fn get_line(&mut self) -> Result<Option<(String, Vec<String>)>, ()> {
        let readline = self.prompt();

        match readline {
            Ok(line) => {
                self.rl.add_history_entry(line.as_ref());

                trace!("line: {:?}", line);
                if is_comment(&line) {
                    return Ok(None)
                }

                let cmd = parse(&line);
                debug!("got {:?}", cmd);

                if cmd.len() == 0 {
                    Ok(None)
                } else {
                    let prog = cmd[0].clone();
                    Ok(Some((prog, cmd)))
                }
            },
            Err(_) => Err(()),
        }
    }

    pub fn run(mut self) {
        loop {
            match self.get_line() {
                Ok(Some((prog, args))) => {
                    debug!("prog: {:?}, args: {:?}", prog, args);
                    self.process(prog, args);
                },
                Ok(None) => (),
                Err(_) => break,
            }
        }
    }
}

fn is_comment(line: &str) -> bool {
    for x in line.chars() {
        match x {
            '#' => return true,
            ' ' => (),
            _   => return false,
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let cmd = parse("foo\\  \\\\bar");
        assert_eq!(cmd, vec!["foo ", "\\bar"]);
    }

    #[test]
    fn test_empty() {
        let cmd = parse("");

        let expected: Vec<String> = Vec::new();
        assert_eq!(expected, cmd);
    }

    #[test]
    fn test_newline() {
        let cmd = parse("\n");

        let expected: Vec<String> = Vec::new();
        assert_eq!(expected, cmd);
    }

    #[test]
    fn test_is_comment() {
        assert_eq!(false, is_comment("hello world"));
        assert_eq!(true, is_comment("#hello world"));
        assert_eq!(false, is_comment("hello #world"));
        assert_eq!(false, is_comment(""));
        assert_eq!(false, is_comment("  "));
        assert_eq!(true, is_comment("  # x"));
    }
}
