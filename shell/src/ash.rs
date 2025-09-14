use boxutils::commands::Command;

use std::env;
use std::io::{self, Write};
use std::process;

use anyhow::{Error, Result};

use crate::built_in::{Action, run_if_exists};

pub struct Ash;

impl Command for Ash {
    fn execute(&self) -> Result<(), Error> {
        let mut path = env::current_dir().unwrap().display().to_string();

        loop {
            let userchar = if boxutils::cross::user::is_admin() {
                '#'
            } else {
                '$'
            };
            print!("{} {} ", path.clone(), userchar);
            let _ = io::stdout().flush();
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let mut full_cmd = input.trim().split_whitespace();
            let command = full_cmd.next().unwrap_or("");
            let arguments: Vec<&str> = full_cmd.collect();

            if let Some(action) = run_if_exists(command.to_string(), arguments.clone()) {
                match action? {
                    Action::Exit => {
                        break;
                    }

                    Action::ChangeDirectory(directory) => {
                        path = directory;
                        env::set_current_dir(&path).unwrap();
                    }

                    Action::Nothing => {}
                }
            } else {
                process::Command::new(command)
                    .args(arguments.clone())
                    .spawn()?
                    .wait()?;
            }
        }

        Ok(())
    }
}
