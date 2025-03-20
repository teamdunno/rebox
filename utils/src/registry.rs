use super::commands::Command;
use std::collections::HashMap;

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

    pub fn get(&self, name: &str) -> Option<&Box<dyn Command>> {
        self.commands.get(name)
    }

    pub fn execute(&self, name: &str) {
        if let Some(command) = self.get(name) {
            command.execute();
        } else {
            println!("Command not found: {}", name);
        }
    }
}

mod tests {
    #[test]
    fn test_register() {
        use super::Command;
        use super::CommandRegistry;
        struct TestCommand;
        impl Command for TestCommand {
            fn execute(&self) {}
        }

        let mut registry = CommandRegistry::new();
        registry.register("test", Box::new(TestCommand));
        assert!(registry.get("test").is_some());
    }

    #[test]
    fn test_execute() {
        use super::Command;
        use super::CommandRegistry;
        struct TestCommand;
        impl Command for TestCommand {
            fn execute(&self) {
                println!("TestCommand executed");
            }
        }

        let mut registry = CommandRegistry::new();
        registry.register("test", Box::new(TestCommand));
        registry.execute("test");
    }
}
