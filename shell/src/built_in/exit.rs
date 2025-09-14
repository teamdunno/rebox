use crate::built_in::Action;
use anyhow::Result;

pub fn exit(_: Vec<&str>) -> Result<Action> {
    Ok(Action::Exit)
}
