use super::boxcmd::Boxcmd;
use boxutils::registry::CommandRegistry;

pub fn get_registry() -> CommandRegistry {
    let mut registry = CommandRegistry::new();

    registry.register("hello", Box::new(coreutils::commands::Hello));
    registry.register("cat", Box::new(coreutils::commands::Cat));
    registry.register("echo", Box::new(coreutils::commands::Echo));
    registry.register("mkdir", Box::new(coreutils::commands::Mkdir));
    registry.register("box", Box::new(Boxcmd));

    registry
}
