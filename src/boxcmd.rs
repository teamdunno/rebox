use super::registry::get_registry;
use anyhow::{Result, bail};
use boxutils::args::ArgParser;
use boxutils::commands::Command;

pub struct Boxcmd;

impl Command for Boxcmd {
    fn execute(&self) -> Result<()> {
        let parser = ArgParser::builder().parse_args("box");

        let registry = get_registry();

        for command in parser.get_normal_args() {
            if command == "box" {
                continue;
            }

            registry.execute(&command)?;
        }

        bail!(
            "No valid command provided. Included commands:\n{}",
            registry.list().join(", ")
        );
    }
}
