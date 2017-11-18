#[macro_use] extern crate boxxy;
extern crate env_logger;

fn example(sh: &mut boxxy::Shell, args: Vec<String>) -> Result<(), boxxy::Error> {
    shprintln!(sh, "The world is your oyster! {:?}", args);
    Ok(())
}

fn main() {
    env_logger::init().unwrap();

    let toolbox = boxxy::Toolbox::new().with(vec![
            ("example", example),
        ]);
    boxxy::Shell::new(toolbox).run()
}
