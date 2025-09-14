use anyhow::{Result, bail};
use boxutils::args::ArgParser;
use boxutils::commands::Command;
use boxutils::encoding::base64;
use std::fs::OpenOptions;
use std::io::Read;

// TODO: Add the -w flag
//       we dont have a way to do text
//       warping in boxutils yet haha

pub struct Base64;

impl Command for Base64 {
    fn execute(&self) -> Result<()> {
        let args = ArgParser::builder()
            .add_flag("-d") // decode flag
            .parse_args("base64");

        let decode = args.get_flag("-d");

        // FIXME: This is jank!
        let mut file: Box<dyn Read> = match &args.get_normal_args()[..] {
            [] => Box::new(std::io::stdin()),
            [file] => Box::new(OpenOptions::new().read(true).open(file).unwrap()),
            _ => bail!("base64: multiple files provided"),
        };

        let mut buffer = String::new();
        while boxutils::input::repl_with_file(&mut file, &mut buffer) {
            let data = if decode {
                base64::decode(buffer.clone())
            } else {
                base64::encode(buffer.clone())
            };

            println!("{}", data);
        }

        Ok(())
    }
}
