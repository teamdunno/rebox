use super::registry::get_registry;
use boxutils::args::ArgParser;
use boxutils::commands::Command;

pub struct Boxcmd;

impl Command for Boxcmd {
    fn execute(&self) {
        let parser = ArgParser::builder().parse_args("box");

        let registry = get_registry();

        for command in parser.get_normal_args() {
            if command == "box" {
                continue;
            }

            registry.execute(&command);
            return;
        }

        println!(
            "No valid command provided. Included commands:\n{}",
            registry.list().join(", ")
        );
    }
}
