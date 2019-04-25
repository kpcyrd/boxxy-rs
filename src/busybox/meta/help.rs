use crate::{Result, Shell, Arguments};

pub fn help(sh: &mut Shell, _args: Arguments) -> Result<()> {
    let mut commands = sh.list_commands();
    commands.sort_unstable();

    for cmd in commands {
        shprintln!(sh, "{:?}", cmd);
    }

    Ok(())
}
