use anyhow::Result;
use boxutils::args::ArgParser;
use boxutils::commands::Command;
use boxutils::cross::user;

pub struct WhoAmI;

impl Command for WhoAmI {
    fn execute(&self) -> Result<()> {
        let args = ArgParser::builder().add_flag("--help").parse_args("whoami");

        if args.get_flag("--help") {
            println!("Usage: whoami");
            return Ok(());
        }

        let username = user::get_username().unwrap_or("unknown".to_string());

        println!("{}", username);

        Ok(())
    }
}
