use super::boxcmd::Boxcmd;
use boxutils::registry::CommandRegistry;

pub fn get_registry() -> CommandRegistry {
    let mut registry = CommandRegistry::new();

    registry.register("hello", Box::new(coreutils::commands::Hello));
    registry.register("cat", Box::new(coreutils::commands::Cat));
    registry.register("echo", Box::new(coreutils::commands::Echo));
    registry.register("mkdir", Box::new(coreutils::commands::Mkdir));
    registry.register("ash", Box::new(shell::ash::Ash));
    registry.register("dd", Box::new(coreutils::commands::Dd));
    registry.register("nproc", Box::new(coreutils::commands::Nproc));
    registry.register("true", Box::new(coreutils::commands::True));
    registry.register("false", Box::new(coreutils::commands::False));
    registry.register(
        "test",
        Box::new(coreutils::commands::Test::without_bracket()),
    );
    registry.register("[", Box::new(coreutils::commands::Test::with_bracket()));
    registry.register("yes", Box::new(coreutils::commands::Yes));
    registry.register("box", Box::new(Boxcmd));

    registry
}
