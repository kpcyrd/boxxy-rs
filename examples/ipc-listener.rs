extern crate libc;
extern crate ctrlc;

use std::io;
use std::fs;
use std::thread;
use std::time::Instant;
use std::sync::{Arc, Mutex};
use std::os::unix::net::UnixStream;
use std::os::unix::net::UnixListener;
use std::io::prelude::*;
use std::os::unix::io::{RawFd, IntoRawFd, FromRawFd};


#[inline]
fn from(fd: RawFd) -> UnixStream {
    unsafe { UnixStream::from_raw_fd(fd) }
}

fn pipe(mut r: impl Read, mut w: impl Write) {
    loop {
        let mut buf = [0; 1024];

        let n = r.read(&mut buf).expect("read");

        // read until EOF
        if n == 0 {
            break;
        }

        w.write(&buf).expect("write");
    }
}

fn ctrlc(exit: Arc<Mutex<Instant>>) {
    let mut exit = exit.lock().unwrap();
    let now = Instant::now();

    if now.duration_since(*exit).as_secs() < 1 {
        std::process::exit(0);
    } else {
        println!("^C twice to exit");
        *exit = now;
    }
}

fn main() {
    let path = "/tmp/boxxy";
    fs::remove_file(path).ok();
    let listener = UnixListener::bind(path).expect("bind");

    let exit = Arc::new(Mutex::new(Instant::now()));
    ctrlc::set_handler(move || {
        ctrlc(exit.clone());
    }).expect("Error setting Ctrl-C handler");

    eprintln!("[*] listening on {:?}...", path);
    let (stream, addr) = listener.accept().expect("accept");

    eprintln!("[+] connected: {:?}", addr);
    let f = stream.into_raw_fd();

    let fd = f.clone();
    let t1 = thread::spawn(move || pipe(from(fd), io::stdout()));
    let fd = f.clone();
    let t2 = thread::spawn(move || pipe(io::stdin(), from(fd)));

    t1.join().unwrap();
    t2.join().unwrap();
}
