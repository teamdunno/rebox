use boxutils::args::ArgParser;
use boxutils::commands::Command;
use boxutils::cross::user;

pub struct WhoAmI;

impl Command for WhoAmI {
    fn execute(&self) {
        let args = ArgParser::builder().add_flag("--help").parse_args("whoami");

        if args.get_flag("--help") {
            println!("Usage: whoami");
            return;
        }

        let username = user::get_username().unwrap_or_else(|| "unknown".to_string());

        println!("{}", username);
    }
}
