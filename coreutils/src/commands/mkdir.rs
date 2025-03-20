use boxutils::commands::Command;
use boxutils::commands::get_args;
use std::fs;
use std::env;

pub struct Mkdir;

impl Command for Mkdir {
    fn execute(&self) {
        let args: Vec<String> = env::args().collect::<Vec<_>>().clone();
        let arguments: Vec<String> = get_args(String::from("mkdir"), args.clone());

        if arguments.len() == 0 {
            panic!(
                "{}",
                String::from(
                    "Usage: mkdir [DIR1] [DIR1] etc. pp. [-p, --parents]"
                )
            );
        }
        for arg in arguments.iter() {
            if arg == "--help" {
                println!(
                    "{}",
                    String::from(
                        "Usage: mkdir [DIR1] [DIR2] etc. pp. [-p, --parents]"
                ));
                return;
            }
        }

        for arg in arguments.iter() {
            if (arg != "-p") & (arg != "--parent") {
                if args.contains(&String::from("-p")) || args.contains(&String::from("--parent")) {
                    let _ = fs::create_dir_all(String::from(arg));
                } else {
                    fs::create_dir(String::from(arg)).unwrap();
                }
            }
        }
    }
}
