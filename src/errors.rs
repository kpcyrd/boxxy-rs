pub use anyhow::{anyhow, bail, Context, Error, Result};
pub use log::{debug, error, info, trace, warn};

#[inline]
pub fn errno() -> Error {
    errno::errno().into()
}
