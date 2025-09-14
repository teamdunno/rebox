mod cd;
mod eval;
mod exit;

use anyhow::Result;

#[derive(Debug)]
pub enum Action {
    Exit,
    ChangeDirectory(String),
    Nothing,
}

fn get_function(command: String) -> Option<fn(Vec<&str>) -> Result<Action>> {
    let registry = [
        ("exit", exit::exit as fn(Vec<&str>) -> Result<Action>),
        ("cd", cd::cd as fn(Vec<&str>) -> Result<Action>),
        ("eval", eval::eval as fn(Vec<&str>) -> Result<Action>),
    ];
    let mut function: Option<fn(Vec<&str>) -> Result<Action>> = None;

    for entry in registry {
        if entry.0 == &command {
            function = Some(entry.1)
        }
    }

    function
}

pub fn run_if_exists(command: String, arguments: Vec<&str>) -> Option<Result<Action>> {
    let function = get_function(command);
    if let Some(function) = function {
        let action = function(arguments);

        Some(action)
    } else {
        None
    }
}
