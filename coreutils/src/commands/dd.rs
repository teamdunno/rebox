use boxutils::commands::Command;
use std::env;
use std::collections::HashMap;
use std::io::{self, Read, Write, BufReader};
use std::fs::File;
use std::time::Instant;

pub struct Dd;

impl Command for Dd {
    fn execute(&self) {
        // dd has its seperate argument parsing
        let mut arguments = HashMap::new();
        let mut blocksize = 512;

        for argument in env::args() {
            if let Some((k, v)) = argument.split_once('=') {
                arguments.insert(k.to_string().to_lowercase(), v.to_string());
            } // don't handle anything that does not follow dd's syntax.
              // TODO: inform about malformed arguments
        }

        if let Some(bs) = arguments.get("bs") {
            let (k, v) = bs.split_at(bs.len() - 1);
            if v.parse::<u64>().is_ok() {
                // assume the bs is specified in bytes,
                // because the last char is a number
                blocksize = bs.parse::<u64>().unwrap()
            } else {
                match v {
                    "K" | "k" => blocksize = k.parse::<u64>().unwrap() * 1024,
                    "M" => blocksize = k.parse::<u64>().unwrap() * 1024 * 1024,
                    "G" => blocksize = k.parse::<u64>().unwrap() * 1024 * 1024 * 1024,
                    "kB" => blocksize = k.parse::<u64>().unwrap() * 1000,
                    "MB" => blocksize = k.parse::<u64>().unwrap() * 1000 * 1000,
                    "GB" => blocksize = k.parse::<u64>().unwrap() * 1000 * 1000 * 1000,
                    _ => {
                        eprintln!("Invalid blocksize specified.");
                        return
                    }
                }
            }
        }

        let mut vecbuf = Vec::with_capacity(blocksize.try_into().unwrap());
        let start = Instant::now();

        if let Some(input) = arguments.get("if") {
            let mut f = BufReader::new(File::open(input).unwrap());
            let _ = f.read_to_end(&mut vecbuf).unwrap();
        } else {
            let _ = io::stdin().read_to_end(&mut vecbuf).unwrap();
        }

        if let Some(output) = arguments.get("of") {
            let buffer = File::create(output);
            let _ = buffer.unwrap().write(&vecbuf);
        } else {
            let _ = io::stdout().write_all(&vecbuf);
        }

        let duration = start.elapsed().as_millis() / 1000;

        let out_blocks = vecbuf.len() as u64 / blocksize;
        let out_remainder = vecbuf.len() as u64 % blocksize;

        println!(
            "{}+{} records in", // TODO: actually calculate records in
            out_blocks,
            out_remainder
        );

        println!(
            "{}+{} records out",
            out_blocks,
            out_remainder
        );

        println!(
            "{} bytes copied, ca. {} seconds",
            vecbuf.len(),
            duration
        )
    }
}
