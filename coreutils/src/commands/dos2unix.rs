use boxutils::args::ArgParser;
use boxutils::commands::Command;
use std::fs::File;
use std::io::{self, BufReader, Read, Write};
use std::process::exit;

pub fn convert(arguments: &ArgParser, d2u: bool) -> Vec<u8> {
    let mut vecbuf = Vec::new();

    if arguments.get_normal_args().len() == 0 {
        let _ = io::stdin().read_to_end(&mut vecbuf);
    }

    for file in arguments.get_normal_args().iter() {
        let mut tmpbuf = Vec::new();
        let mut read = BufReader::new(File::open(file).unwrap());
        let _ = read.read_to_end(&mut tmpbuf);
        let _ = vecbuf.append(&mut tmpbuf);
    }

    if d2u {
        vecbuf.retain(
            |x|
            *x != b'\r'
        );
    } else {
        let mut tmpbuf = Vec::new();
        vecbuf.iter().enumerate().for_each(|(i, &b)| {
            if b == b'\n' && i > 0 && vecbuf[i - 1] != b'\r' {
                tmpbuf.push(b'\r');
            }
            tmpbuf.push(b);
        });
        vecbuf = tmpbuf;
    }

    vecbuf
}

pub struct Dos2Unix;

impl Command for Dos2Unix {
    fn execute(&self) {
        let args = ArgParser::builder()
            .add_flag("-u")
            .add_flag("-d")
            .add_flag("--help")
            .parse_args("dos2unix");

        let mut dos2unix = true;

        if args.get_flag("-u") {
            dos2unix = true;
        }

        if args.get_flag("-d") {
            dos2unix = false;
        }

        if args.get_flag("--help") {
            println!("Usage: dos2unix [-d] [-u] [FILE]");
            print!("\n");
            println!("-d: unix2dos");
            println!("-u: dos2unix (default)");
            exit(0);
        }

        let result = convert(&args, dos2unix);

        if args.get_normal_args().len() < 1 {
            let _ = io::stdout().write_all(&result);
        } else {
            let _ = std::fs::write(args.get_normal_args()[0].clone(), &result);
        }
    }
}
