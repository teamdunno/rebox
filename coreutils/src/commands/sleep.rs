use boxutils::{args::ArgParser, commands::Command};

pub struct Sleep;

impl Command for Sleep {
    fn execute(&self) {
        let args = ArgParser::builder().parse_args("sleep");
        let mut sleep_for = 0;

        for arg in args.get_normal_args() {
            if arg.chars().last().unwrap().is_numeric() {
                sleep_for += arg.parse::<i32>().unwrap();
            } else {
                let multiplier = match arg.chars().last().unwrap() {
                    's' => 1,
                    'm' => 60,
                    'h' => 3600,
                    'd' => 86400,
                    _ => {
                        println!("Invalid time interval '{}'", arg);
                        return;
                    }
                };

                sleep_for += arg[..arg.len() - 1].parse::<i32>().unwrap() * multiplier;
            }
        }

        if sleep_for == 0 {
            println!("Usage: sleep [N]...");
            return;
        }

        std::thread::sleep(std::time::Duration::from_secs(sleep_for as u64));
    }
}
