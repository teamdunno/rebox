use super::registry::get_registry;
use boxutils::commands::Command;
use std::env;

pub struct Boxcmd;

impl Command for Boxcmd {
    fn execute(&self) {
        let args: Vec<String> = env::args().collect();
        let registry = get_registry();
        let command = &args.get(1);
        if let Some(cmd) = command {
            registry.execute(cmd);
            return;
        }
        println!(
            "No command provided. Included commands:\n{}",
            registry.list().join(", ")
        );
    }
}
