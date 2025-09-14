use anyhow::Result;
use boxutils::commands::Command;
use boxutils::commands::get_args;
use std::env;
use std::fs::File;
use std::io::{self, BufReader, Read, Write};

pub struct Cat;

impl Command for Cat {
    fn execute(&self) -> Result<()> {
        let args: Vec<String> = env::args().collect::<Vec<_>>().clone();
        let arguments: Vec<String> = get_args("cat".to_owned(), args);
        let mut vecbuf = Vec::new();

        if arguments.len() == 0 {
            let _ = io::stdin().read_to_end(&mut vecbuf);
        }

        for file in arguments.iter() {
            let mut tmpbuf = Vec::new();
            let mut read = BufReader::new(File::open(file).unwrap());
            let _ = read.read_to_end(&mut tmpbuf);
            let _ = vecbuf.append(&mut tmpbuf);
        }

        io::stdout().write_all(&vecbuf)?;

        Ok(())
    }
}
