use crate::{Shell, Arguments};
use crate::errors::*;
use std::env;

pub fn pwd(sh: &mut Shell, _args: Arguments) -> Result<()> {
    let path = env::current_dir()?;
    let path = path.to_str().unwrap().to_owned();
    shprintln!(sh, "{:?}", path);
    Ok(())
}
