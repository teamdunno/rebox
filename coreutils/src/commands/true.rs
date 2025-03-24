use boxutils::commands::Command;
use std::process::exit;

pub struct True;

impl Command for True {
    fn execute(&self) {
        exit(0);
    }
}
