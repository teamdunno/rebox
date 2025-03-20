use boxutils::commands::Command;

pub struct Echo;

impl Command for Echo {
    fn execute(&self) {
        let args: Vec<String> = std::env::args().collect::<Vec<_>>();
        let arguments: Vec<String> = boxutils::commands::get_args(String::from("echo"), args);
        let message = &arguments.join(" ");
        println!("{}", message);
    }
}
