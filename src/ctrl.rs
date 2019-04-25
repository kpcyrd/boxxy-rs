use crate::Toolbox;
#[cfg(feature="readline")]
use crate::completer::CmdCompleter;
#[cfg(feature="network")]
use crate::crypto::OwnedTlsStream;
#[cfg(feature="readline")]
use rustyline::{self, Editor};

use std::fs::File;
use std::sync::{Arc, Mutex};
use bufstream::BufStream;
use std::io;
use std::io::prelude::*;
use std::fmt::Debug;
#[cfg(all(unix, feature="network"))]
use std::os::unix::net::UnixStream;
#[cfg(unix)]
use std::os::unix::io::{RawFd, AsRawFd};


#[derive(Debug)]
pub enum PromptError {
    Io(io::Error),
    Eof,
    #[cfg(feature="readline")]
    Other(rustyline::error::ReadlineError),
}

#[cfg(feature="readline")]
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


/// Wraps a Read object and a Write object into a Read/Write object.
#[derive(Debug)]
pub struct RW<R: Read, W: Write>(R, W);

#[cfg(unix)]
impl<R: Read+AsRawFd, W: Write+AsRawFd> RW<R, W> {
    #[inline]
    pub fn as_raw_fd(&self) -> (RawFd, RawFd) {
        let r = self.0.as_raw_fd();
        let w = self.1.as_raw_fd();
        (r, w)
    }
}

impl<R: Read, W: Write> Read for RW<R, W> {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.read(buf)
    }
}

impl<R: Read, W: Write> Write for RW<R, W> {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.1.write(buf)
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        self.1.flush()
    }
}


pub trait R: Read + Debug {}
impl<T> R for T where T: Read + Debug {}

pub trait W: Write + Debug {}
impl<T> W for T where T: Write + Debug {}


/// The interface that the [`Shell`] uses.
///
/// [`Shell`]: ../shell/struct.Shell.html
#[derive(Debug)]
pub enum Interface {
    #[cfg(feature="readline")]
    Fancy((io::Stdin, io::Stdout, Editor<CmdCompleter>)),
    Stdio(BufStream<RW<io::Stdin, io::Stdout>>),
    File(BufStream<RW<File, File>>),
    RWPair(BufStream<RW<Box<R>, Box<W>>>),
    #[cfg(feature="network")]
    Tls(Box<BufStream<OwnedTlsStream>>),
    #[cfg(all(unix, feature="network"))]
    Ipc(BufStream<UnixStream>),
    Dummy(Vec<u8>),
}

impl Interface {
    #[allow(unused_variables)]
    pub fn default(toolbox: &Arc<Mutex<Toolbox>>) -> Interface {
        #[cfg(feature="readline")]
        let ui = Interface::fancy(toolbox.clone());
        #[cfg(not(feature="readline"))]
        let ui = Interface::stdio();

        ui
    }

    #[cfg(feature="readline")]
    pub fn fancy(toolbox: Arc<Mutex<Toolbox>>) -> Interface {
        let mut rl = Editor::new();
        let c = CmdCompleter::new(toolbox);
        rl.set_helper(Some(c));

        Interface::Fancy((io::stdin(), io::stdout(), rl))
    }

    pub fn stdio() -> Interface {
        Interface::Stdio(BufStream::new(RW(io::stdin(), io::stdout())))
    }

    // TODO: this can fail
    pub fn file(input: Option<File>, output: Option<File>) -> Interface {
        let input = input.unwrap_or_else(|| File::open("/dev/null").unwrap());
        let output = output.unwrap_or_else(|| File::open("/dev/null").unwrap());

        Interface::File(BufStream::new(RW(input, output)))
    }

    pub fn rw_pair(input: Box<R>, output: Box<W>) -> Interface {
        Interface::RWPair(BufStream::new(RW(input, output)))
    }

    pub fn dummy() -> Interface {
        Interface::Dummy(Vec::new())
    }

    pub fn readline_raw<RW: BufRead + Write>(prompt: &str, x: &mut RW) -> Result<String, PromptError> {
        x.write_all(prompt.as_bytes())?;
        x.flush()?;

        let mut buf = String::new();
        x.read_line(&mut buf)?;

        if buf.is_empty() {
            return Err(PromptError::Eof)
        }

        let buf = buf.trim_end().to_owned();

        Ok(buf)
    }

    pub fn readline(&mut self, prompt: &str) -> Result<String, PromptError> {
        match *self {
            #[cfg(feature="readline")]
            Interface::Fancy(ref mut x) => {
                let buf = x.2.readline(prompt)?;
                Ok(buf)
            },
            Interface::Stdio(ref mut x) => Self::readline_raw(prompt, x),
            Interface::File(ref mut x) => Self::readline_raw(prompt, x),
            Interface::RWPair(ref mut x) => Self::readline_raw(prompt, x),
            #[cfg(feature="network")]
            Interface::Tls(ref mut x) => Self::readline_raw(prompt, x),
            #[cfg(all(unix, feature="network"))]
            Interface::Ipc(ref mut x) => Self::readline_raw(prompt, x),
            Interface::Dummy(ref mut _x) => unimplemented!(),
        }
    }

    #[cfg(feature="readline")]
    pub fn add_history_entry(&mut self, line: &str) {
        if let Interface::Fancy(ref mut x) = *self {
            x.2.add_history_entry(line);
        }
    }

    #[cfg(unix)]
    #[inline]
    pub fn pipe(&mut self) -> Option<(RawFd, RawFd, RawFd)> {
        match *self {
            // this connects the real stdio automatically
            #[cfg(feature="readline")]
            Interface::Fancy(_) => None,
            Interface::Stdio(ref ui) => {
                let (r, w) = ui.get_ref().as_raw_fd();
                Some((r, w, w))
            },
            Interface::File(ref ui) => {
                let (r, w) = ui.get_ref().as_raw_fd();
                Some((r, w, w))
            },
            Interface::RWPair(_) => None,
            // NOTE: not supported yet
            #[cfg(feature="network")]
            Interface::Tls(_) => None,
            #[cfg(all(unix, feature="network"))]
            Interface::Ipc(ref ui) => {
                let fd = ui.get_ref().as_raw_fd();
                Some((fd, fd, fd))
            },
            Interface::Dummy(_) => None,
        }
    }
}

impl Read for Interface {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match *self {
            #[cfg(feature="readline")]
            Interface::Fancy(ref mut x) => x.0.read(buf),
            Interface::Stdio(ref mut x) => x.read(buf),
            Interface::File(ref mut x) => x.read(buf),
            Interface::RWPair(ref mut x) => x.read(buf),
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
            #[cfg(feature="readline")]
            Interface::Fancy(ref mut x) => x.1.write(buf),
            Interface::Stdio(ref mut x) => x.write(buf),
            Interface::File(ref mut x) => x.write(buf),
            Interface::RWPair(ref mut x) => x.write(buf),
            #[cfg(feature="network")]
            Interface::Tls(ref mut x) => x.write(buf),
            #[cfg(all(unix, feature="network"))]
            Interface::Ipc(ref mut x) => x.write(buf),
            Interface::Dummy(ref mut x) => x.write(buf),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match *self {
            #[cfg(feature="readline")]
            Interface::Fancy(ref mut x) => x.1.flush(),
            Interface::Stdio(ref mut x) => x.flush(),
            Interface::File(ref mut x) => x.flush(),
            Interface::RWPair(ref mut x) => x.flush(),
            #[cfg(feature="network")]
            Interface::Tls(ref mut x) => x.flush(),
            #[cfg(all(unix, feature="network"))]
            Interface::Ipc(ref mut x) => x.flush(),
            Interface::Dummy(ref mut x) => x.flush(),
        }
    }
}
