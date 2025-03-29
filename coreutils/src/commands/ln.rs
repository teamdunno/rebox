use boxutils::args::ArgParser;
use boxutils::commands::Command;
use std::env;
use std::ops::Not;
use std::process::exit;

pub struct Ln;

impl Command for Ln {
    fn execute(&self) {
        let mut to_be_linked = String::from("");
        let mut destination = String::from("");
        let family = env::consts::FAMILY;
        let args = ArgParser::builder()
            .add_flag("-s")
            .add_flag("-f")
            .add_flag("-n")
            .add_flag("-b")
            .add_flag("-v")
            .add_option("-S")
            .parse_args("ln");
        fn help() {
            println!("Lorem ipsum dolor sit amet.");
            exit(1)
        }
        if args.get_normal_args().len() != 2 {
            help();
        } else {
            to_be_linked = args.get_normal_args()[0].clone();
            destination = args.get_normal_args()[1].clone();
        }
        if ((family == "unix") || (family == "windows")).not() {
            eprintln!("OS family is unknown: {}. Aborting.", family);
            exit(1);
        }

        #[cfg(unix)]
        fn main(args: ArgParser, to_be_linked: String, destination: String) {
            use std::os::unix;
            use std::fs;

            if args.get_flag("-v"){
                println!("'{}' -> '{}'", to_be_linked, destination)
            }
            if args.get_flag("-f") {
                if fs::metadata(String::from(&destination)).is_ok() {
                    if fs::metadata(String::from(&destination)).unwrap().is_dir() {
                        fs::remove_dir_all(String::from(&destination)).unwrap()
                    } else {
                        fs::remove_file(String::from(&destination)).unwrap()
                    }
                }
            }

            if args.get_flag("-b") {
                let suffix = args.get_option("-S").unwrap_or("~");
                let new_filename = format!("{}{}", destination, suffix);
                let _ = fs::rename(&destination, new_filename);
            }
            if args.get_flag("-s") {
                if args.get_flag("-n") {
                    let _ = unix::fs::symlink(String::from(&to_be_linked), String::from(&destination)).unwrap();
                } else {
                    let mut metadata = fs::metadata(&to_be_linked).unwrap();
                    let file = &to_be_linked;
                    let mut new_file = file.clone();
                    while metadata.is_symlink() {
                        new_file = String::from(fs::read_link(new_file.to_string()).unwrap().into_os_string().into_string().unwrap());
                        metadata = fs::metadata(&new_file).unwrap()
                    }
                    let _ = unix::fs::symlink(String::from(new_file), String::from(&destination)).unwrap();
                }
                exit(0)
            } else {
                if args.get_flag("-n") {
                    let _ = fs::hard_link(String::from(&to_be_linked), String::from(&destination)).unwrap();
                } else {
                    let mut metadata = fs::metadata(&to_be_linked).unwrap();
                    let file = &to_be_linked;
                    let mut new_file = file.clone();
                    while metadata.is_symlink() {
                        new_file = String::from(fs::read_link(new_file.to_string()).unwrap().into_os_string().into_string().unwrap());
                        metadata = fs::metadata(&new_file).unwrap()
                    }
                    let _ = fs::hard_link(String::from(&new_file), String::from(&destination)).unwrap();
                }
            }
        }

        #[cfg(windows)]
        fn main(args: ArgParser, to_be_linked: String, destination: String) {
            use std::os::windows;
            use std::fs;
            // TODO: basic code at line 77
        }

        let _ = main(args, to_be_linked, destination);

//            #[cfg(windows)]
//            fn windows(to_be_linked: String, destination: String) {
//                use std::os::windows;
//                use std::fs;
//                let attr = fs::metadata(args.get_normal_args()[0].clone()).unwrap();
//                if attr.is_dir() {
//                    let _ = windows::fs::symlink_dir(args.get_normal_args()[0].clone(), args.get_normal_args()[1].clone());
//                } else {
//                    let _ = windows::fs::symlink_file(args.get_normal_args()[0].clone(), args.get_normal_args()[1].clone());
//                }
//            }
//            #[cfg(windows)]
//            windows(to_be_linked, destination);
//        }
    }
}
