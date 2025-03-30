use std::path::Path;

use boxutils::{args::ArgParser, commands::Command};

pub struct Dirname;

impl Command for Dirname {
    fn execute(&self) {
        let args = ArgParser::builder()
            .add_flag("--help")
            .parse_args("dirname");

        if args.get_normal_args().len() != 1 || args.get_flag("--help") {
            println!("Usage: dirname FILENAME");
            return;
        }

        // note(teesh): we have already checked for argnums, so we're fine :D
        let normal_args = args.get_normal_args();
        let file = normal_args.get(0).unwrap();
        let path = Path::new(file);

        if let Some(parent) = path.parent() {
            let mut to_print = format!("{}", parent.display());

            // note(teesh): this is stupid, but thats how the box busies
            if to_print == "" {
                to_print = ".".into();
            }

            println!("{}", to_print);
        } else {
            println!("dirname: could not get parent")
        }
    }
}
