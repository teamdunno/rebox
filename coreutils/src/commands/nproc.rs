use boxutils::commands::Command;
use boxutils::args::ArgParser;
use std::env;
use num_cpus::{get, get_physical};

pub struct Nproc;

impl Command for Nproc {
    fn execute(&self) {
        let raw_args = env::args().collect::<Vec<_>>();
        let args = ArgParser::builder()
            .add_flag("--help")
            .add_flag("--ignore")
            .add_flag("--all")
            .parse("nproc", raw_args);
        let mut ignore: u64 = 0;
        let mut all = false;
        if args.get_flag("--help") {
            println!(
"
Usage: nproc [--all] [ignore=N]
Prints the number of available CPUs to stdout.

    --all       List all installed CPUs
    --ignore=N, --ignore N  Ignore N CPUs
"
            );
            return
        }

        if args.get_flag("--all") {
            all = true;
        }

        if args.get_flag("--ignore") {
            ignore = args.get_option("--ignore").unwrap().parse().unwrap();
        }

        for argument in args.get_normal_args() {
            if let Some((k, v)) = argument.split_once('=') {
                if k == "--ignore" {
                    ignore = v.parse().unwrap();
                }
            }
        }

        if all {
            println!("{}", get() as u64 - ignore) // TODO: actually make `--all` do something
        } else {
            println!("{}", get() as u64 - ignore)
        }
    }
}
