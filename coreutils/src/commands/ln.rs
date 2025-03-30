use boxutils::args::ArgParser;
use boxutils::commands::Command;
use std::fs;

pub struct Ln;

impl Command for Ln {
    fn execute(&self) {
        let args = ArgParser::builder()
            .add_flag("--help")
            .add_flag("-s")
            .add_flag("-f")
            .add_flag("-n")
            .add_flag("-b")
            .add_flag("-v")
            .add_option("-S")
            .parse_args("ln");

        let mut dereference = true;
        let help = args.get_normal_args().len() != 2 || args.get_flag("--help");
        if help {
            println!("Usage: ln [-sfnbtv] [-S SUF] TARGET LINK");
            return;
        }

        let to_be_linked = args.get_normal_args()[0].clone();
        let destination = args.get_normal_args()[1].clone();

        if args.get_flag("-v") {
            println!("{} -> {}", to_be_linked, destination);
        }

        if args.get_flag("-f") {
            if fs::exists(&destination).unwrap() {
                if fs::metadata(&destination).unwrap().is_dir() {
                    fs::remove_dir_all(&destination).unwrap();
                } else {
                    fs::remove_file(&destination).unwrap();
                }
            }
        }

        if args.get_flag("-n") {
            dereference = false;
        }

        if args.get_flag("-b") {
            let suffix = args.get_option("-S").unwrap_or("~");
            let new_filename = format!("{}{}", destination, suffix);
            let _ = fs::rename(&destination, &new_filename);
        }

        if args.get_flag("-s") {
            let symlink_result = boxutils::cross::fs::symlink(to_be_linked, destination, dereference);

            if let Err(e) = symlink_result {
                eprintln!("ln: failed to create symlink: {}", e);
            }
        } else {
            if let Err(e) = boxutils::cross::fs::hard_link(to_be_linked, destination, dereference) {
                eprintln!("ln: failed to create hard link: {}", e);
            }
        }
    }
}
