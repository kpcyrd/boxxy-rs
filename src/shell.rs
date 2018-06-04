//! The interactive shell.

use busybox;
use clap;

use Error;
use ErrorKind;
use ctrl::{Interface, PromptError};
pub use ffi::ForeignCommand;
use std::io;
use std::io::prelude::*;
use std::sync::Arc;
use std::sync::Mutex;
use std::collections::HashMap;

#[cfg(unix)]
use std::os::unix::io::RawFd;

#[cfg(unix)]
use ffi::daemonize;


#[derive(Clone)]
pub enum Command {
    Native(NativeCommand),
    Foreign(ForeignCommand),
}

pub type NativeCommand = fn(&mut Shell, Vec<String>) -> Result<(), Error>;


impl Command {
    pub fn run(&self, mut sh: &mut Shell, args: Vec<String>) -> Result<(), Error> {
        use self::Command::*;
        match *self {
            Native(ref func)  => func(&mut sh, args),
            Foreign(ref func) => func.run(args),
        }
    }

    #[cfg(unix)]
    pub fn daemonized(&self, sh: &Shell, args: Vec<String>) -> Result<(), Error> {
        daemonize(sh.daemon_clone(), self.clone(), args)
    }

    #[cfg(not(unix))]
    pub fn daemonized(&self, sh: &Shell, args: Vec<String>) -> Result<(), Error> {
        // not implemented, execute in same process
        self.run(&mut sh.daemon_clone(), args)
    }
}


impl From<NativeCommand> for Command {
    fn from(cmd: NativeCommand) -> Command {
        Command::Native(cmd)
    }
}

impl From<ForeignCommand> for Command {
    fn from(cmd: ForeignCommand) -> Command {
        Command::Foreign(cmd)
    }
}


/// The set of registered commands.
#[derive(Default)]
pub struct Toolbox(HashMap<String, Command>);

impl Toolbox {
    /// Create an empty toolbox.
    ///
    /// ```
    /// use boxxy::Toolbox;
    ///
    /// let toolbox = Toolbox::empty();
    /// ```
    #[inline]
    pub fn empty() -> Toolbox {
        Toolbox(HashMap::new())
    }

    /// Create a toolbox that contains the default builtin commands.
    ///
    /// ```
    /// use boxxy::Toolbox;
    ///
    /// let toolbox = Toolbox::new();
    /// ```
    #[inline]
    pub fn new() -> Toolbox {
        let mut toolbox = Toolbox::empty();
        toolbox.insert_many_native(vec![
            ("cat"          , busybox::cat),
            ("cd"           , busybox::cd),
            ("downgrade"    , busybox::downgrade),
            ("echo"         , busybox::echo),
            ("exec"         , busybox::exec),
            ("grep"         , busybox::grep),
            ("help"         , busybox::help),
            ("ls"           , busybox::ls),
            ("mkdir"        , busybox::mkdir),
            ("pwd"          , busybox::pwd),
            ("rm"           , busybox::rm),
        ]);

        #[cfg(unix)]
        toolbox.insert_many_native(vec![
            ("chmod"        , busybox::chmod),
            ("chown"        , busybox::chown),
            ("chroot"       , busybox::chroot),
            ("jit"          , busybox::jit),
            ("id"           , busybox::id),
            ("setgroups"    , busybox::setgroups),
            ("setgid"       , busybox::setgid),
            ("setuid"       , busybox::setuid),
            ("seteuid"      , busybox::seteuid),
        ]);

        #[cfg(target_os="linux")]
        toolbox.insert_many_native(vec![
            ("caps"         , busybox::caps),
            ("mount"        , busybox::mount),
            ("setresgid"    , busybox::setresgid),
            ("setresuid"    , busybox::setresuid),
            ("setreuid"     , busybox::setreuid),
        ]);

        #[cfg(feature="archives")]
        toolbox.insert_many_native(vec![
            ("tar"          , busybox::tar),
        ]);

        #[cfg(feature="network")]
        toolbox.insert_many_native(vec![
            ("curl"         , busybox::curl),
            ("revshell"     , busybox::revshell),
            #[cfg(unix)]
            ("ipcshell"     , busybox::ipcshell),
        ]);

        toolbox
    }

    /// Get a command by its name.
    ///
    /// ```
    /// use boxxy::Toolbox;
    ///
    /// let toolbox = Toolbox::new();
    /// println!("command: {:?}", toolbox.get("cat").is_some());
    /// ```
    #[inline]
    pub fn get(&self, key: &str) -> Option<&Command> {
        self.0.get(key)
    }

    /// List available commands.
    ///
    /// ```
    /// use boxxy::Toolbox;
    ///
    /// let toolbox = Toolbox::new();
    /// println!("commands: {:?}", toolbox.keys());
    /// ```
    #[inline]
    pub fn keys(&self) -> Vec<String> {
        self.0
            .keys()
            .map(|x| x.to_owned())
            .collect()
    }

    /// Insert a command into the toolbox.
    ///
    /// ```
    /// #[macro_use] extern crate boxxy;
    /// use boxxy::{Toolbox, Command};
    ///
    /// fn example(sh: &mut boxxy::Shell, args: Vec<String>) -> Result<(), boxxy::Error> {
    ///     shprintln!(sh, "The world is your oyster! {:?}", args);
    ///     Ok(())
    /// }
    ///
    /// fn main() {
    ///     let mut toolbox = Toolbox::new();
    ///     toolbox.insert("example", Command::Native(example));
    ///     println!("commands: {:?}", toolbox.keys());
    /// }
    /// ```
    #[inline]
    pub fn insert<I: Into<String>>(&mut self, key: I, func: Command) {
        self.0.insert(key.into(), func);
    }

    /// Insert many commands into the toolbox.
    ///
    /// ```
    /// #[macro_use] extern crate boxxy;
    /// use boxxy::{Toolbox, Command};
    ///
    /// fn example1(sh: &mut boxxy::Shell, _args: Vec<String>) -> Result<(), boxxy::Error> {
    ///     shprintln!(sh, "example1");
    ///     Ok(())
    /// }
    ///
    /// fn example2(sh: &mut boxxy::Shell, _args: Vec<String>) -> Result<(), boxxy::Error> {
    ///     shprintln!(sh, "example2");
    ///     Ok(())
    /// }
    ///
    /// fn main() {
    ///     let mut toolbox = Toolbox::new();
    ///     toolbox.insert_many(vec![
    ///         ("example1", Command::Native(example1)),
    ///         ("example2", Command::Native(example2)),
    ///     ]);
    ///     println!("commands: {:?}", toolbox.keys());
    /// }
    /// ```
    #[inline]
    pub fn insert_many<I: Into<String>>(&mut self, commands: Vec<(I, Command)>) {
        for (key, func) in commands {
            self.insert(key, func);
        }
    }

    /// Insert many [`NativeCommand`]s into the toolbox.
    ///
    /// [`NativeCommand`]: struct.NativeCommand.html
    ///
    /// ```
    /// #[macro_use] extern crate boxxy;
    /// use boxxy::Toolbox;
    ///
    /// fn example1(sh: &mut boxxy::Shell, _args: Vec<String>) -> Result<(), boxxy::Error> {
    ///     shprintln!(sh, "example1");
    ///     Ok(())
    /// }
    ///
    /// fn example2(sh: &mut boxxy::Shell, _args: Vec<String>) -> Result<(), boxxy::Error> {
    ///     shprintln!(sh, "example2");
    ///     Ok(())
    /// }
    ///
    /// fn main() {
    ///     let mut toolbox = Toolbox::new();
    ///     toolbox.insert_many_native(vec![
    ///         ("example1", example1),
    ///         ("example2", example2),
    ///     ]);
    ///     println!("commands: {:?}", toolbox.keys());
    /// }
    /// ```
    #[inline]
    pub fn insert_many_native<I: Into<String>>(&mut self, commands: Vec<(I, NativeCommand)>) {
        for (key, func) in commands {
            self.insert(key, func.into());
        }
    }

    /// Builder pattern to create a toolbox with custom commands.
    ///
    /// ```
    /// #[macro_use] extern crate boxxy;
    /// use boxxy::Toolbox;
    ///
    /// fn example(sh: &mut boxxy::Shell, args: Vec<String>) -> Result<(), boxxy::Error> {
    ///     shprintln!(sh, "The world is your oyster! {:?}", args);
    ///     Ok(())
    /// }
    ///
    /// fn main() {
    ///    let toolbox = Toolbox::new().with(vec![
    ///            ("example", example),
    ///        ]);
    ///    println!("commands: {:?}", toolbox.keys());
    /// }
    /// ```
    #[inline]
    pub fn with<I: Into<String>>(mut self, commands: Vec<(I, NativeCommand)>) -> Toolbox {
        self.insert_many_native(commands);
        self
    }
}


/// The struct that keeps track of the user interface.
pub struct Shell {
    ui: Interface,
    toolbox: Arc<Mutex<Toolbox>>,
}

impl Read for Shell {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.ui.read(buf)
    }
}

impl Write for Shell {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.ui.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.ui.flush()
    }
}

impl Shell {
    /// Initializes a shell. Takes a [`Toolbox`] that contains the available
    /// commands. The toolbox is also used to configure tab completion.
    ///
    /// [`Toolbox`]: struct.Toolbox.html
    ///
    /// ```
    /// use boxxy::{Shell, Toolbox};
    ///
    /// let toolbox = Toolbox::new();
    /// let shell = Shell::new(toolbox);
    /// ```
    pub fn new(toolbox: Toolbox) -> Shell {
        let toolbox = Arc::new(Mutex::new(toolbox));

        let ui = Interface::default(&toolbox);

        Shell {
            ui,
            toolbox,
        }
    }

    /// Replace the readline interface with a plain stdin/stdout interface.
    ///
    /// ```
    /// use boxxy::{Shell, Toolbox};
    ///
    /// let toolbox = Toolbox::new();
    /// let mut shell = Shell::new(toolbox);
    /// shell.downgrade();
    /// shell.run();
    /// ```
    #[inline]
    pub fn downgrade(&mut self) {
        match self.ui {
            #[cfg(feature="readline")]
            Interface::Fancy(_) => {
                self.ui = Interface::stdio();
            },
            _ => shprintln!(self, "[-] interface is already downgraded"),
        }
    }

    #[inline]
    pub fn list_commands(&self) -> Vec<String> {
        let toolbox = self.toolbox.lock().unwrap();
        toolbox.keys()
    }

    #[inline]
    pub fn hotswap(&mut self, ui: Interface) {
        self.ui = ui;
    }

    #[cfg(unix)]
    #[inline]
    pub fn pipe(&mut self) -> Option<(RawFd, RawFd, RawFd)> {
        self.ui.pipe()
    }

    /// Insert a [`Command`] into the [`Toolbox`].
    ///
    /// [`Toolbox`]: struct.Toolbox.html
    /// [`Command`]: enum.Command.html
    /// ```
    /// use boxxy::{self, Shell, Command, Toolbox};
    ///
    /// let toolbox = Toolbox::empty();
    /// let mut shell = Shell::new(toolbox);
    /// shell.insert("ls", Command::Native(boxxy::busybox::ls));
    /// ```
    #[inline]
    pub fn insert<I: Into<String>>(&mut self, name: I, command: Command) {
        let mut toolbox = self.toolbox.lock().unwrap();
        toolbox.insert(name, command);
    }

    fn process(&mut self, cmd: InputCmd) {
        debug!("cmd: {:?}", cmd);

        let result: Option<Command> = {
            let toolbox = self.toolbox.lock().unwrap();
            match toolbox.get(&cmd.prog) {
                Some(x) => Some(x.clone()),
                None => None,
            }
        };

        let result = match (result, cmd.bg) {
            (Some(func), true) => func.daemonized(&self, cmd.args),
            (Some(func), false) => func.run(self, cmd.args),
            (None, _) => Err(ErrorKind::Args(clap::Error {
                message: String::from("\u{1b}[1;31merror:\u{1b}[0m unknown command"),
                kind: clap::ErrorKind::MissingRequiredArgument,
                info: None,
            }).into()),
        };

        if let Err(err) = result {
            match *err.kind() {
                ErrorKind::Args(ref err)    => shprintln!(self, "{}", err.message),
                _                           => shprintln!(self, "error: {:?}", err),
            }
        }
    }

    fn daemon_clone(&self) -> Shell {
        let toolbox = self.toolbox.clone();
        let ui = Interface::default(&toolbox);
        Shell {
            ui,
            toolbox,
        }
    }

    #[inline]
    fn prompt(&mut self) -> Result<String, PromptError> {
        self.ui.readline(" [%]> ")
    }

    #[inline]
    fn get_line(&mut self) -> Result<Option<InputCmd>, ()> {
        let readline = self.prompt();

        match readline {
            Ok(line) => {
                #[cfg(feature="readline")]
                self.ui.add_history_entry(line.as_ref());
                Ok(parse_line(&line))
            },
            Err(_) => Err(()),
        }
    }

    #[inline]
    pub fn exec_once(&mut self, line: &str) {
        if let Some(cmd) = parse_line(line) {
            self.process(cmd);
        }
    }

    /// Run the input loop. This doesn't return until the shell is exited.
    ///
    /// ```
    /// use boxxy::{Shell, Toolbox};
    ///
    /// let toolbox = Toolbox::new();
    /// let mut shell = Shell::new(toolbox);
    ///
    /// // run the loop
    /// shell.run();
    /// ```
    pub fn run(&mut self) {
        loop {
            match self.get_line() {
                Ok(Some(cmd)) => self.process(cmd),
                Ok(None) => (),
                Err(_) => break,
            }
        }
    }
}


#[inline]
fn tokenize(line: &str) -> Vec<String> {
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
                if !token.is_empty() {
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

    if !token.is_empty() {
        cmd.push(token);
    }

    cmd
}


#[derive(Debug, PartialEq)]
pub struct InputCmd {
    prog: String,
    args: Vec<String>,
    bg: bool,
}


#[inline]
fn parse_line(line: &str) -> Option<InputCmd> {
    trace!("line: {:?}", line);
    if is_comment(&line) {
        return None;
    }

    let (bg, line) = if line.ends_with(" &") {
        (true, &line[..line.len()-2])
    } else {
        (false, line)
    };

    let cmd = tokenize(&line);
    debug!("got {:?}", cmd);

    if cmd.is_empty() {
        None
    } else {
        let prog = cmd[0].clone();
        Some(InputCmd {
            prog,
            args: cmd,
            bg,
        })
    }
}


#[inline]
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
        let cmd = tokenize("foo\\  \\\\bar");
        assert_eq!(cmd, vec!["foo ", "\\bar"]);
    }

    #[test]
    fn test_empty() {
        let cmd = tokenize("");

        let expected: Vec<String> = Vec::new();
        assert_eq!(expected, cmd);
    }

    #[test]
    fn test_newline() {
        let cmd = tokenize("\n");

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

    #[test]
    fn test_bg() {
        let cmd = parse_line("foo bar &");
        assert_eq!(Some(InputCmd {
            prog: "foo".to_string(),
            args: vec!["foo".to_string(), "bar".to_string()],
            bg: true,
        }), cmd);
    }
}
