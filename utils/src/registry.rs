use super::commands::Command;
use std::collections::HashMap;

use anyhow::{Error, Result, bail};

pub struct CommandRegistry {
    commands: HashMap<String, Box<dyn Command>>,
}

impl CommandRegistry {
    pub fn new() -> Self {
        CommandRegistry {
            commands: HashMap::new(),
        }
    }

    pub fn register(&mut self, name: &str, command: Box<dyn Command>) {
        self.commands.insert(name.to_string(), command);
    }

    pub fn list(&self) -> Vec<String> {
        let mut bufvec = Vec::new();
        for (key, _value) in self.commands.iter() {
            bufvec.push(String::from(key))
        }
        bufvec
    }

    pub fn get(&self, name: &str) -> Option<&Box<dyn Command>> {
        self.commands.get(name)
    }

    pub fn execute(&self, name: &str) -> Result<(), Error> {
        if let Some(command) = self.get(name) {
            command.execute()?;
        } else {
            bail!("Command not found: {}", name);
        }

        Ok(())
    }
}

mod tests {
    #[allow(unused_imports)] // why the heck is rust saying it's unused??
    use anyhow::{Error, Result};

    #[test]
    fn test_register() {
        use super::Command;
        use super::CommandRegistry;

        struct TestCommand;
        impl Command for TestCommand {
            fn execute(&self) -> Result<(), Error> {
                Ok(())
            }
        }

        let mut registry = CommandRegistry::new();
        registry.register("test", Box::new(TestCommand));
        assert!(registry.get("test").is_some());
    }

    #[test]
    fn test_execute() -> Result<(), Error> {
        use super::Command;
        use super::CommandRegistry;

        struct TestCommand;
        impl Command for TestCommand {
            fn execute(&self) -> Result<(), Error> {
                println!("TestCommand executed");

                Ok(())
            }
        }

        let mut registry = CommandRegistry::new();
        registry.register("test", Box::new(TestCommand));
        registry.execute("test")?;

        Ok(())
    }
}
