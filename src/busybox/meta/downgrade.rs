use crate::{Result, Shell, Arguments};

pub fn downgrade(sh: &mut Shell, _args: Arguments) -> Result<()> {
    sh.downgrade();
    Ok(())
}
