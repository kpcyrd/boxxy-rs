use boxxy::shprintln;

fn example(sh: &mut boxxy::Shell, args: Vec<String>) -> boxxy::Result<()> {
    shprintln!(sh, "The world is your oyster! {:?}", args);
    Ok(())
}

fn main() {
    env_logger::init();

    let toolbox = boxxy::Toolbox::new().with(vec![("example", example)]);
    boxxy::Shell::new(toolbox).run()
}
