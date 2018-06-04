use ::{Result, Shell, Arguments};

pub fn help(sh: &mut Shell, _args: Arguments) -> Result<()> {
    for cmd in sh.list_commands() {
        shprintln!(sh, "{:?}", cmd);
    }

    Ok(())
}
