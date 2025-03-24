use boxutils::args::ArgParser;
use boxutils::commands::Command;

pub struct Pwd;

impl Command for Pwd {
    fn execute(&self) {
        let args = ArgParser::builder().add_flag("--help").parse_args("yes");

        if args.get_flag("--help") {
            println!("Usage: pwd");
            return;
        }

        let pwd = std::env::current_dir().unwrap();
        println!("{}", pwd.display());
    }
}
