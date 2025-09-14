use anyhow::Result;
use boxutils::args::ArgParser;
use boxutils::commands::Command;

pub struct Yes;

impl Command for Yes {
    fn execute(&self) -> Result<()> {
        let args = ArgParser::builder().add_flag("--help").parse_args("yes");

        if args.get_flag("--help") {
            println!("Usage: yes [STRING]");
            return Ok(());
        }

        let string = if args.get_normal_args().is_empty() {
            String::from("y")
        } else {
            args.get_normal_args().join(" ")
        };

        loop {
            println!("{}", string);
        }
    }
}
