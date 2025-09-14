use crate::built_in::Action;
use anyhow::{Result, bail};
use std::process::Command;

pub fn eval(arguments: Vec<&str>) -> Result<Action> {
    if arguments.len() < 1 {
        bail!("eval expects **one or more** arguments");
    }

    Command::new(arguments[0])
        .args(&arguments[1..])
        .spawn()?
        .wait()?;
    Ok(Action::Nothing)
}

