use anyhow::{Result, bail};
use boxutils::{args::ArgParser, commands::Command};

pub struct Sleep;

impl Command for Sleep {
    fn execute(&self) -> Result<()> {
        let args = ArgParser::builder().parse_args("sleep");
        let mut sleep_for = 0;

        for arg in args.get_normal_args() {
            if arg.chars().last().unwrap().is_numeric() {
                sleep_for += arg.parse::<i32>()?;
            } else {
                let multiplier = match arg.chars().last().unwrap() {
                    's' => 1,
                    'm' => 60,
                    'h' => 3600,
                    'd' => 86400,
                    _ => {
                        bail!("Invalid time interval '{}'", arg);
                    }
                };

                sleep_for += arg[..arg.len() - 1].parse::<i32>()? * multiplier;
            }
        }

        if sleep_for == 0 {
            println!("Usage: sleep [N]...");
            return Ok(());
        }

        std::thread::sleep(std::time::Duration::from_secs(sleep_for as u64));

        Ok(())
    }
}
