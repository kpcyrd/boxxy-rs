use clap::{App, Arg};

fn main() {
    env_logger::init();

    let matches = App::new("boxxy-ipc")
        .arg(Arg::with_name("path").required(true))
        .get_matches();

    let path = matches.value_of("path").unwrap();

    let toolbox = boxxy::Toolbox::new();
    let mut shell = boxxy::Shell::new(toolbox);

    shell.exec_once(&format!("ipcshell -- {}", path)); // TODO: need better interface
    shell.run()
}
