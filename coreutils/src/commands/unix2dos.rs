use crate::commands::dos2unix::convert;
use boxutils::args::ArgParser;
use boxutils::commands::Command;
use std::io::{self, Write};
use std::process::exit;

pub struct Unix2Dos;

impl Command for Unix2Dos {
    fn execute(&self) {
        let args = ArgParser::builder()
            .add_flag("-u")
            .add_flag("-d")
            .add_flag("--help")
            .parse_args("unix2dos");

        let mut dos2unix = false;

        if args.get_flag("-u") {
            dos2unix = true;
        }

        if args.get_flag("-d") {
            dos2unix = false;
        }

        if args.get_flag("--help") {
            println!("Usage: unix2dos [-d] [-u] [FILE]");
            println!("\n");
            println!("-d: unix2dos (default)");
            println!("-u: dos2unix");
            exit(0);
        }

        let result = convert(&args, dos2unix);

        if args.get_normal_args().len() < 1 {
            let _ = io::stdout().write_all(&result);
        } else {
            let _ = std::fs::write(args.get_normal_args()[0].clone(), &result);
        }

    }
}
