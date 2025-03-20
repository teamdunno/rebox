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
        let filename = &arguments[0];

        let mut read = BufReader::new(File::open(filename).unwrap());
        let mut vecbuf = Vec::new();
        let _ = read.read_to_end(&mut vecbuf);
        let _ = io::stdout().write_all(&vecbuf);
    }
}
