#![cfg_attr(not(unix), allow(unused_imports, dead_code))]

use clap::{App, AppSettings, Arg};
use std::fs;
use std::io;
use std::io::prelude::*;
#[cfg(unix)]
use std::os::unix::io::{FromRawFd, IntoRawFd, RawFd};
#[cfg(unix)]
use std::os::unix::net::{UnixListener, UnixStream};
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

#[inline]
#[cfg(unix)]
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

        w.write_all(&buf[..n]).expect("write");
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

#[cfg(unix)]
fn main() {
    let matches = App::new("ipc-listener")
        .setting(AppSettings::DisableVersion)
        .arg(
            Arg::with_name("path")
                .help("unix domain socket path")
                .required(true),
        )
        .arg(Arg::with_name("script").help("execute this script after bind"))
        .get_matches();

    let path = matches.value_of("path").unwrap();
    let script = matches.value_of("script");

    fs::remove_file(path).ok();
    let listener = UnixListener::bind(path).expect("bind");

    let exit = Arc::new(Mutex::new(Instant::now()));
    ctrlc::set_handler(move || {
        ctrlc(exit.clone());
    })
    .expect("Error setting Ctrl-C handler");

    eprintln!("[*] listening on {:?}...", path);

    if let Some(script) = script {
        eprintln!("[*] running {:?}", script);
        Command::new("sh")
            .args(["-c", script])
            .status()
            .expect("exec");
    }

    let (stream, addr) = listener.accept().expect("accept");
    eprintln!("[+] connected: {:?}", addr);

    let f = stream.into_raw_fd();

    let t1 = thread::spawn(move || pipe(from(f), io::stdout()));
    let t2 = thread::spawn(move || pipe(io::stdin(), from(f)));

    t1.join().unwrap();
    t2.join().unwrap();
}

#[cfg(not(unix))]
fn main() {
    panic!("unsupported platform");
}
