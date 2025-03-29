use crate::boxcmd::Boxcmd;
use boxutils::registry::CommandRegistry;

macro_rules! register {
    ($registry:expr, { $($cmd_name:expr => $cmd:expr),* }) => {
        $(
            $registry.register($cmd_name, Box::new($cmd));
        )*
    };
}

pub fn get_registry() -> CommandRegistry {
    let mut registry = CommandRegistry::new();

    register!(registry, {
        "hello" => coreutils::commands::Hello,
        "cat" => coreutils::commands::Cat,
        "echo" => coreutils::commands::Echo,
        "mkdir" => coreutils::commands::Mkdir,
        "ash" => shell::ash::Ash,
        "dd" => coreutils::commands::Dd,
        "nproc" => coreutils::commands::Nproc,
        "true" => coreutils::commands::True,
        "false" => coreutils::commands::False,
        "test" => coreutils::commands::Test::without_bracket(),
        "[" => coreutils::commands::Test::with_bracket(),
        "yes" => coreutils::commands::Yes,
        "pwd" => coreutils::commands::Pwd,
        "sleep" => coreutils::commands::Sleep,
        "whoami" => coreutils::commands::WhoAmI,
        "hostname" => coreutils::commands::Hostname,
        "tee" => coreutils::commands::Tee,
        "base64" => coreutils::commands::Base64,
        "base32" => coreutils::commands::Base32,
        "seq" => coreutils::commands::Seq,
        "env" => coreutils::commands::Env,
        "ln" => coreutils::commands::Ln,
        "box" => Boxcmd
    });

    registry
}
