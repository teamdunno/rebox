use boxutils::args::ArgParser;
use boxutils::commands::Command;
use std::fs::OpenOptions;
use std::io::Write;

// TODO: Add the -i flag to ignore SIGINT.
//       Not done yet because we want this to be
//       Windows-compatible

pub struct Tee;

impl Command for Tee {
    fn execute(&self) {
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
                eprintln!("tee: unable to open file: {}", file);
            }
        }

        let mut buffer = String::new();
        while std::io::stdin().read_line(&mut buffer).unwrap_or(0) > 0 {
            for output in &mut writes {
                let _ = output.write_all(buffer.as_bytes());
                let _ = output.flush();
            }
            buffer.clear();
        }
    }
}
