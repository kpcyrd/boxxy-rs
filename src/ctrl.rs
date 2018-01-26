use Toolbox;
use shell::CmdCompleter;
#[cfg(feature="network")]
use crypto::OwnedTlsStream;
use rustyline::{self, Editor};

use std::sync::Arc;
use std::sync::Mutex;
#[cfg(feature="network")]
use bufstream::BufStream;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
#[cfg(all(unix, feature="network"))]
use std::os::unix::net::UnixStream;


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
    Fancy((io::Stdin, io::Stdout, Editor<CmdCompleter>)),
    Stdio((BufReader<io::Stdin>, io::Stdout)),
    #[cfg(feature="network")]
    Tls(BufStream<OwnedTlsStream>),
    #[cfg(all(unix, feature="network"))]
    Ipc(BufStream<UnixStream>),
    Dummy(Vec<u8>),
}

impl Interface {
    pub fn fancy(toolbox: Arc<Mutex<Toolbox>>) -> Interface {
        let mut rl = Editor::new();
        let c = CmdCompleter::new(toolbox);
        rl.set_completer(Some(c));

        Interface::Fancy((io::stdin(), io::stdout(), rl))
    }

    pub fn stdio() -> Interface {
        Interface::Stdio((BufReader::new(io::stdin()), io::stdout()))
    }

    pub fn dummy() -> Interface {
        Interface::Dummy(Vec::new())
    }

    pub fn readline(&mut self, prompt: &str) -> Result<String, PromptError> {
        match *self {
            Interface::Fancy(ref mut x) => {
                let buf = x.2.readline(prompt)?;
                Ok(buf)
            },
            Interface::Stdio(ref mut x) => {
                x.1.write(prompt.as_bytes())?;
                x.1.flush()?;

                let mut buf = String::new();
                x.0.read_line(&mut buf)?;

                if buf.len() == 0 {
                    return Err(PromptError::Eof)
                }

                let buf = buf.trim_right().to_owned(); // TODO

                Ok(buf)
            },
            #[cfg(feature="network")]
            Interface::Tls(ref mut x) => {
                x.write(prompt.as_bytes())?;
                x.flush()?;

                let mut buf = String::new();
                x.read_line(&mut buf)?;

                if buf.len() == 0 {
                    return Err(PromptError::Eof)
                }

                let buf = buf.trim_right().to_owned(); // TODO

                Ok(buf)
            },
            #[cfg(all(unix, feature="network"))]
            Interface::Ipc(ref mut x) => {
                x.write(prompt.as_bytes())?;
                x.flush()?;

                let mut buf = String::new();
                x.read_line(&mut buf)?;

                if buf.len() == 0 {
                    return Err(PromptError::Eof)
                }

                let buf = buf.trim_right().to_owned(); // TODO

                Ok(buf)
            },
            Interface::Dummy(ref mut _x) => unimplemented!(),
        }
    }

    pub fn add_history_entry(&mut self, line: &str) -> bool {
        match *self {
            Interface::Fancy(ref mut x) => x.2.add_history_entry(line),
            _ => true,
        }
    }
}

impl Read for Interface {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match *self {
            Interface::Fancy(ref mut x) => x.0.read(buf),
            Interface::Stdio(ref mut x) => x.0.read(buf),
            #[cfg(feature="network")]
            Interface::Tls(ref mut x) => x.read(buf),
            #[cfg(all(unix, feature="network"))]
            Interface::Ipc(ref mut x) => x.read(buf),
            Interface::Dummy(ref mut _x) => unimplemented!(),
        }
    }
}

impl Write for Interface {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match *self {
            Interface::Fancy(ref mut x) => x.1.write(buf),
            Interface::Stdio(ref mut x) => x.1.write(buf),
            #[cfg(feature="network")]
            Interface::Tls(ref mut x) => x.write(buf),
            #[cfg(all(unix, feature="network"))]
            Interface::Ipc(ref mut x) => x.write(buf),
            Interface::Dummy(ref mut x) => x.write(buf),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match *self {
            Interface::Fancy(ref mut x) => x.1.flush(),
            Interface::Stdio(ref mut x) => x.1.flush(),
            #[cfg(feature="network")]
            Interface::Tls(ref mut x) => x.flush(),
            #[cfg(all(unix, feature="network"))]
            Interface::Ipc(ref mut x) => x.flush(),
            Interface::Dummy(ref mut x) => x.flush(),
        }
    }
}
