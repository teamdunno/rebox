use crate::built_in::Action;

pub fn cd(arguments: Vec<&str>) -> Action {
    if arguments.len() < 1 {
        panic!("cd expects **one** argument");
    }

    Action::ChangeDirectory(arguments[0].to_owned().clone())
}
