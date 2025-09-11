use crate::built_in::Action;
use std::process::Command;

pub fn eval(arguments: Vec<&str>) -> Action {
    if arguments.len() < 1 {
        panic!("eval expects **one or more** arguments");
    }

    let output = Command::new(arguments[0]).args(&arguments[1..]).spawn();
    match output {
        Ok(mut output) => {
            output.wait().expect("failed to wait for process exit");
        }
        Err(err) => println!("{:?}", err)
    }
    Action::Nothing
}