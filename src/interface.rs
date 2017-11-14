use Toolbox;
use shell::CmdCompleter;
use rustyline::{self, Editor};

use std::sync::Arc;
use std::sync::Mutex;
use bufstream::BufStream;
use std::fs::File; // TODO: remove
use std::io;
use std::io::prelude::*;
use std::io::Stdout;
use std::io::BufReader;


#[derive(Debug)]
pub enum PromptError {
    Io(io::Error),
    Eof,
    Other(rustyline::error::ReadlineError),
}

impl From<rustyline::error::ReadlineError> for PromptError {
    fn from(err: rustyline::error::ReadlineError) -> PromptError {
        use rustyline::error::ReadlineError;
        match err {
            ReadlineError::Io(err) => PromptError::Io(err),
            ReadlineError::Eof => PromptError::Eof,
            x => PromptError::Other(x),
        }
    }
}

impl From<io::Error> for PromptError {
    fn from(err: io::Error) -> PromptError {
        PromptError::Io(err)
    }
}


#[derive(Debug)]
pub enum Interface {
    Fancy((Stdout, Editor<CmdCompleter>)),
    Stdio((BufReader<io::Stdin>, io::Stdout)),
    Stream(BufStream<File>),
}

impl Interface {
    pub fn fancy(toolbox: Arc<Mutex<Toolbox>>) -> Interface {
        let mut rl = Editor::new();
        let c = CmdCompleter::new(toolbox);
        rl.set_completer(Some(c));

        Interface::Fancy((io::stdout(), rl))
    }

    pub fn stdio() -> Interface {
        Interface::Stdio((BufReader::new(io::stdin()), io::stdout()))
    }

    pub fn readline(&mut self, prompt: &str) -> Result<String, PromptError> {
        match *self {
            Interface::Fancy(ref mut x) => {
                let buf = x.1.readline(prompt)?;
                Ok(buf)
            },
            Interface::Stdio(ref mut x) => {
                x.1.write(prompt.as_bytes()).unwrap();
                x.1.flush().unwrap();

                let mut buf = String::new();
                x.0.read_line(&mut buf)?;

                if buf.len() == 0 {
                    return Err(PromptError::Eof)
                }

                let buf = buf.trim_right().to_owned(); // TODO

                Ok(buf)
            },
            Interface::Stream(ref mut x) => {
                x.write(prompt.as_bytes()).unwrap();
                x.flush().unwrap();

                let mut buf = String::new();
                x.read_line(&mut buf)?;

                if buf.len() == 0 {
                    return Err(PromptError::Eof)
                }

                let buf = buf.trim_right().to_owned(); // TODO

                Ok(buf)
            },
        }
    }

    pub fn add_history_entry(&mut self, line: &str) -> bool {
        match *self {
            Interface::Fancy(ref mut x) => x.1.add_history_entry(line),
            _ => true,
        }
    }
}

impl Write for Interface {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match *self {
            Interface::Fancy(ref mut x) => x.0.write(buf),
            Interface::Stdio(ref mut x) => x.1.write(buf),
            Interface::Stream(ref mut x) => x.write(buf),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match *self {
            Interface::Fancy(ref mut x) => x.0.flush(),
            Interface::Stdio(ref mut x) => x.1.flush(),
            Interface::Stream(ref mut x) => x.flush(),
        }
    }
}
