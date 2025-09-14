use crate::built_in::Action;
use anyhow::{Result, bail};

pub fn cd(arguments: Vec<&str>) -> Result<Action> {
    if arguments.len() < 1 {
        bail!("cd expects **one** argument");
    }

    Ok(Action::ChangeDirectory(arguments[0].to_owned().clone()))
}
