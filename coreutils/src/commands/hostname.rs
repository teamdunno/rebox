use boxutils::args::ArgParser;
use boxutils::commands::Command;

pub struct Hostname;

impl Command for Hostname {
    fn execute(&self) {
        let args = ArgParser::builder()
            .add_flag("--help")
            .parse_args("hostname");

        if args.get_flag("--help") {
            println!("Usage: hostname")
        }

        let hostname = match hostname::get() {
            Ok(hn) => hn,
            Err(_) => "unknown".into(),
        };

        println!("{}", hostname.to_string_lossy());
    }
}
