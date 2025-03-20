use std::env;
use std::mem;
use std::path::Path;

pub fn being_called_as() -> String {
    let args = env::args().collect::<Vec<String>>();
    let exe_path = args[0].clone();
    let path = Path::new(&exe_path);
    let being_called = path.file_name().unwrap().to_str().unwrap().to_string();
    let formatted = being_called.replace(".exe", "");
    formatted
}

pub fn get_args(commandname: String, args: Vec<String>) -> Vec<String> {
    let mut arguments: Vec<String> = args.clone();
    let exe_name = being_called_as();

    let _ = mem::replace(&mut arguments[0], exe_name);

    if let Some(num) = arguments.iter().position(|x| *x == commandname) {
        arguments.drain(..=num);
    };

    arguments
}

pub trait Command {
    fn execute(&self);
}
