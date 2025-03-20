use boxutils::commands::Command;

pub struct Hello;

impl Command for Hello {
    fn execute(&self) {
        println!("Hello, world!");
    }
}
