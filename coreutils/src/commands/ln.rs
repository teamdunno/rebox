use boxutils::args::ArgParser;
use boxutils::commands::Command;
use std::fs;
#[cfg(unix)]
use std::os::unix::fs::symlink;
#[cfg(windows)]
use std::os::windows::fs::{symlink_dir, symlink_file};
use std::path::Path;

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
            if fs::metadata(&destination).is_ok() {
                if fs::metadata(&destination).unwrap().is_dir() {
                    fs::remove_dir_all(&destination).unwrap();
                } else {
                    fs::remove_file(&destination).unwrap();
                }
            }
        }

        if args.get_flag("-n") {
            if Path::new(&destination).exists() {
                println!("ln: {} exists, stopping...", destination);
                return;
            }
        }

        if args.get_flag("-b") {
            let suffix = args.get_option("-S").unwrap_or("~");
            let new_filename = format!("{}{}", destination, suffix);
            let _ = fs::rename(&destination, &new_filename);
        }

        if args.get_flag("-s") {
            #[cfg(unix)]
            {
                if let Err(e) = symlink(&to_be_linked, &destination) {
                    eprintln!("ln: failed to create symlink: {}", e);
                }
            }

            #[cfg(windows)]
            {
                let target_metadata = fs::metadata(&to_be_linked).unwrap();
                let symlink_result = if target_metadata.is_dir() {
                    symlink_dir(&to_be_linked, &destination)
                } else {
                    symlink_file(&to_be_linked, &destination)
                };

                if let Err(e) = symlink_result {
                    eprintln!("ln: failed to create symlink: {}", e);
                }
            }
        } else {
            if let Err(e) = fs::hard_link(&to_be_linked, &destination) {
                eprintln!("ln: failed to create hard link: {}", e);
            }
        }
    }
}
