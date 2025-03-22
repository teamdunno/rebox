use boxutils::commands::Command;
use std::env;
use std::io::{self, ErrorKind, Write};
use std::path::Path;
use std::process::Command as stdCommand;

pub struct Ash;

impl Command for Ash {
    fn execute(&self) {
        loop {
            let path = env::current_dir();
            print!(
                "{} $ ", // TODO: display "#" if root
                path.expect("unknown").display()
            );
            let _ = io::stdout().flush();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let mut full_cmd = input.trim().split_whitespace();
            let command = full_cmd.next().unwrap();
            let mut arguments = full_cmd;
            match command {
                "exit" => return,
                "cd" => {
                    let new_path = arguments.next();
                    let new_path = Path::new(new_path.unwrap());
                    let _ = env::set_current_dir(&new_path).is_ok();
                }
                command => {
                    let out = stdCommand::new(command).args(arguments).spawn();

                    match out {
                        Ok(mut out) => {
                            let _ = out.wait();
                        }
                        Err(err) => match err.kind() {
                            ErrorKind::NotFound => {
                                eprintln!("ash: {}: not found", command);
                            }
                            ErrorKind::PermissionDenied => {
                                eprintln!("ash: {}: permission denied", command);
                            }
                            _ => {
                                eprintln!("ash: uncaught error: {}", err);
                            }
                        },
                    }
                }
            }
        }
    }
}
