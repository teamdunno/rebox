use boxutils::args::Args;
use boxutils::commands::Command;
use std::env;
use std::fs;

pub struct Mkdir;

impl Command for Mkdir {
    fn execute(&self) {
        let raw_args: Vec<String> = env::args().collect::<Vec<_>>();
        let args = Args::new("mkdir", raw_args);

        if args.get_args().len() == 0 {
            panic!(
                "{}",
                String::from("Usage: mkdir [DIR1] [DIR1] etc. pp. [-p, --parents]")
            );
        }

        if args.get_flag("--help") {
            println!("Usage: mkdir [DIR1] [DIR2] etc. pp. [-p, --parents]");
            return;
        }

        let parented = args.get_flag("-p") || args.get_flag("--parents");

        let to_create = args
            .get_args()
            .into_iter()
            .filter(|x| !x.starts_with("-"))
            .collect::<Vec<_>>();

        for dir in to_create {
            if parented {
                let _ = fs::create_dir_all(dir);
            } else {
                let _ = fs::create_dir(dir);
            }
        }
    }
}
