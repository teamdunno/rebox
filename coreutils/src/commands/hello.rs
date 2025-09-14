use anyhow::Result;
use boxutils::commands::Command;

pub struct Hello;

impl Command for Hello {
    fn execute(&self) -> Result<()> {
        println!("Hello, world!");

        Ok(())
    }
}
