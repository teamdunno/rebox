use anyhow::{Result, bail};
use boxutils::args::ArgParser;
use boxutils::commands::Command;
use std::fs;

pub struct Mkdir;

impl Command for Mkdir {
    fn execute(&self) -> Result<()> {
        let args = ArgParser::builder()
            .add_flag("-p")
            .add_flag("--parents")
            .add_flag("--help")
            .parse_args("mkdir");

        if args.get_flag("--help") {
            println!("Usage: mkdir [DIR1] [DIR2] etc. pp. [-p, --parents]");
            return Ok(());
        }

        if args.get_normal_args().len() == 0 {
            bail!("Usage: mkdir [DIR1] [DIR1] etc. pp. [-p, --parents]");
        }

        let parented = args.get_flag("-p") || args.get_flag("--parents");

        let to_create = args.get_normal_args();

        for dir in to_create {
            if parented {
                fs::create_dir_all(dir)?;
            } else {
                fs::create_dir(dir)?;
            }
        }

        Ok(())
    }
}
