extern crate libc;
extern crate ctrlc;
extern crate clap;

use std::io;
use std::fs;
use std::thread;
use std::time::Instant;
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::os::unix::net::UnixStream;
use std::os::unix::net::UnixListener;
use std::io::prelude::*;
use std::os::unix::io::{RawFd, IntoRawFd, FromRawFd};

use clap::{App, Arg, AppSettings};


#[inline]
fn from(fd: RawFd) -> UnixStream {
    unsafe { UnixStream::from_raw_fd(fd) }
}

fn pipe(mut r: impl Read, mut w: impl Write) {
    let mut buf = [0; 1024];

    loop {
        let n = r.read(&mut buf).expect("read");

        // read until EOF
        if n == 0 {
            break;
        }

        w.write(&buf[..n]).expect("write");
        w.flush().expect("flush");
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
    let matches = App::new("ipc-listener")
        .setting(AppSettings::DisableVersion)
        .arg(Arg::with_name("path")
            .help("unix domain socket path")
            .required(true)
        )
        .arg(Arg::with_name("script")
            .help("execute this script after bind")
        )
        .get_matches();

    let path = matches.value_of("path").unwrap();
    let script = matches.value_of("script");

    fs::remove_file(path).ok();
    let listener = UnixListener::bind(path).expect("bind");

    let exit = Arc::new(Mutex::new(Instant::now()));
    ctrlc::set_handler(move || {
        ctrlc(exit.clone());
    }).expect("Error setting Ctrl-C handler");

    eprintln!("[*] listening on {:?}...", path);

    if let Some(script) = script {
        eprintln!("[*] running {:?}", script);
        Command::new("sh")
            .args(&["-c", script])
            .status().expect("exec");
    }

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
