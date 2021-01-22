use crate::{Shell, Arguments};
use crate::errors::*;

pub fn downgrade(sh: &mut Shell, _args: Arguments) -> Result<()> {
    sh.downgrade();
    Ok(())
}
