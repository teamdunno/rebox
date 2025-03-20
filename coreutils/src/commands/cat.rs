use boxutils::commands::Command;
use boxutils::commands::get_args;
use std::env;
use std::fs::File;
use std::io::{self, BufReader, Read, Write};

pub struct Cat;

impl Command for Cat {
    fn execute(&self) {
        let args: Vec<String> = env::args().collect::<Vec<_>>().clone();
        let arguments: Vec<String> = get_args(String::from("cat"), args);
        let mut vecbuf = Vec::new();

        for file in arguments.iter() {
            let mut tmpbuf = Vec::new();
            let mut read = BufReader::new(File::open(file).unwrap());
            let _ = read.read_to_end(&mut tmpbuf);
            let _ = vecbuf.append(&mut tmpbuf);
        }

        let _ = io::stdout().write_all(&vecbuf);
    }
}
