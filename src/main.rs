mod boxcmd;
mod registry;

use anyhow::{Error, Result};
use boxutils::commands::being_called_as;

fn main() -> Result<(), Error> {
    let utility_name = being_called_as();
    registry::get_registry().execute(&utility_name)?;

    Ok(())
}
