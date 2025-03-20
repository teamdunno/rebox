use std::mem;

pub fn get_args(commandname: String, args: Vec<String>) -> Vec<String> {
    let mut arguments: Vec<String> = args.clone();
    let cmd = arguments[0].clone();

    let _ = mem::replace(
        &mut arguments[0],
        cmd
        .replace(".exe", "")
        .replace("./", "")
    );

    if let Some(num) = arguments.iter().position(|x| *x == commandname) {
        arguments.drain(..=num);
    };

    arguments
}

pub trait Command {
    fn execute(&self);
}
