use boxutils::args::ArgParser;
use boxutils::commands::Command;
use std::env;

pub struct WhoAmI;

impl Command for WhoAmI {
    fn execute(&self) {
        let args = ArgParser::builder().add_flag("--help").parse_args("whoami");

        if args.get_flag("--help") {
            println!("Usage: whoami");
            return;
        }

        let username = env::var("USER") // Unix
            .or_else(|_| env::var("USERNAME")) // Windows
            .unwrap_or_else(|_| "unknown".to_string());

        println!("{}", username);
    }
}
