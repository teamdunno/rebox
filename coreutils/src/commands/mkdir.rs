use boxutils::args::ArgParser;
use boxutils::commands::Command;
use std::env;
use std::fs;

pub struct Mkdir;

impl Command for Mkdir {
    fn execute(&self) {
        let raw_args: Vec<String> = env::args().collect::<Vec<_>>();
        let args = ArgParser::builder()
            .add_flag("-p")
            .add_flag("--parents")
            .add_flag("--help")
            .parse("mkdir", raw_args);

        if args.get_flag("--help") {
            println!("Usage: mkdir [DIR1] [DIR2] etc. pp. [-p, --parents]");
            return;
        }

        if args.get_normal_args().len() == 0 {
            panic!("Usage: mkdir [DIR1] [DIR1] etc. pp. [-p, --parents]");
        }

        let parented = args.get_flag("-p") || args.get_flag("--parents");

        let to_create = args.get_normal_args();

        for dir in to_create {
            if parented {
                let _ = fs::create_dir_all(dir);
            } else {
                let _ = fs::create_dir(dir);
            }
        }
    }
}
