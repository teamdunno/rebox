use boxutils::commands::Command;
use std::process::exit;

pub struct False;

impl Command for False {
    fn execute(&self) {
        exit(1);
    }
}
