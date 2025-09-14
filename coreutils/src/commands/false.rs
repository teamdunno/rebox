use anyhow::Result;
use boxutils::commands::Command;
use std::process::exit;

pub struct False;

impl Command for False {
    fn execute(&self) -> Result<()> {
        exit(1);
    }
}
