mod cd;
mod exit;
mod eval;

#[derive(Debug)]
pub enum Action {
    Exit,
    ChangeDirectory(String),
    Nothing
}

fn get_function(command: String) -> Option<fn(Vec<&str>) -> Action> {
    let registry = [
        ("exit", exit::exit as fn(Vec<&str>) -> Action),
        ("cd", cd::cd as fn(Vec<&str>) -> Action),
        ("eval", eval::eval as fn(Vec<&str>) -> Action),
    ];
    let mut function: Option<fn(Vec<&str>) -> Action> = None;

    for entry in registry {
        if entry.0 == &command {
            function = Some(entry.1)
        }
    }

    function
}

pub fn run_if_exists(command: String, arguments: Vec<&str>) -> Option<Action> {
    let function = get_function(command);
    if let Some(function) = function {
        let action = function(arguments);

        Some(action)
    } else {
        None
    }
}
