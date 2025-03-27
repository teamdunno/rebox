use boxutils::args::ArgParser;
use boxutils::commands::Command;
use boxutils::encoding::base32;
use std::fs::OpenOptions;
use std::io::Read;

// TODO: Add the -w flag
//       we dont have a way to do text
//       warping in boxutils yet haha

pub struct Base32;

impl Command for Base32 {
    fn execute(&self) {
        let args = ArgParser::builder()
            .add_flag("-d") // decode flag
            .parse_args("base32");

        let decode = args.get_flag("-d");

        // FIXME: This is jank!
        let mut file: Box<dyn Read> = match &args.get_normal_args()[..] {
            [] => Box::new(std::io::stdin()),
            [file] => Box::new(OpenOptions::new().read(true).open(file).unwrap()),
            _ => panic!("base32: multiple files provided"),
        };

        let mut buffer = String::new();
        while boxutils::input::repl_with_file(&mut file, &mut buffer) {
            let data = if decode {
                base32::decode(buffer.clone())
            } else {
                base32::encode(buffer.clone())
            };

            println!("{}", data);
        }
    }
}
