use anyhow::{Result, bail};
use boxutils::args::ArgParser;
use boxutils::commands::Command;
use std::fs::OpenOptions;
use std::io::Write;

// TODO: Add the -i flag to ignore SIGINT.
//       Not done yet because we want this to be
//       Windows-compatible

pub struct Tee;

impl Command for Tee {
    fn execute(&self) -> Result<()> {
        let args = ArgParser::builder()
            .add_flag("--help")
            .add_flag("-a")
            .parse_args("tee");

        if args.get_flag("--help") {
            println!("Usage: tee -a [FILE]...");
        }

        let append = args.get_flag("-a");
        let files = args.get_normal_args();
        let mut writes: Vec<Box<dyn Write>> = vec![Box::new(std::io::stdout())];

        for file in files {
            let this_file = OpenOptions::new()
                .create(true)
                .write(true)
                .append(append)
                .open(&file);

            if let Ok(this_file) = this_file {
                writes.push(Box::new(this_file));
            } else {
                bail!("tee: unable to open file: {}", file);
            }
        }

        let mut buffer = String::new();
        while boxutils::input::repl(&mut buffer) {
            for output in &mut writes {
                output.write_all(buffer.as_bytes())?;
                output.flush()?;
            }
            buffer.clear();
        }

        Ok(())
    }
}
