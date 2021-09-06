pub use log::{debug, info, warn, error, trace};
pub use anyhow::{Error, Context, Result, anyhow, bail};

#[inline]
pub fn errno() -> Error {
    errno::errno().into()
}
