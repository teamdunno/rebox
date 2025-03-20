use boxutils::registry::CommandRegistry;
use super::boxcmd::Boxcmd;

pub fn get_registry() -> CommandRegistry {
    let mut registry = CommandRegistry::new();

    registry.register("hello", Box::new(coreutils::commands::Hello));
    registry.register("box", Box::new(Boxcmd));

    registry
}
