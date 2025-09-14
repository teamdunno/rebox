use anyhow::Result;
use boxutils::args::ArgParser;
use boxutils::commands::Command;

pub struct Pwd;

impl Command for Pwd {
    fn execute(&self) -> Result<()> {
        let args = ArgParser::builder().add_flag("--help").parse_args("yes");

        if args.get_flag("--help") {
            println!("Usage: pwd");
            return Ok(());
        }

        let pwd = std::env::current_dir()?;
        println!("{}", pwd.display());

        Ok(())
    }
}
