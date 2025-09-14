use anyhow::Result;
use boxutils::args::ArgParser;
use boxutils::commands::Command;

// note(teesh): this command uses technically unsafe stuff,
//
//              HOWEVER: the rust docs state that if you are
//              running the things we're doing on a single-threaded
//              application (which rebox, and env are), then you
//              are fine!

pub struct Env;

impl Env {
    pub fn clean_unwanted(to_clean: Vec<String>) {
        for var in to_clean {
            unsafe {
                std::env::remove_var(var);
            }
        }
    }

    pub fn maybe_clean_env(maybe: bool) {
        if !maybe {
            return;
        }

        Env::clean_unwanted(std::env::vars().map(|(x, _)| x).collect());
    }

    pub fn print_all_vars() {
        for (k, v) in std::env::vars() {
            println!("{}={}", k, v);
        }
    }
}

impl Command for Env {
    fn execute(&self) -> Result<()> {
        let args = ArgParser::builder()
            .add_flag("-i")
            .add_flag("-0")
            .add_option_list("-u")
            .set_seperator("--")
            .parse_args("env");

        let remove_from_env: Vec<String> = args
            .get_option_list("-u")
            .unwrap_or_default()
            .iter()
            .map(|a| a.to_string())
            .collect();
        let null_terminate = args.get_flag("-0");
        let clean_env = args.get_flag("-i") | args.get_flag("-");
        let to_run = args.get_normal_args();

        Env::clean_unwanted(remove_from_env);
        Env::maybe_clean_env(clean_env);

        if to_run.is_empty() {
            Env::print_all_vars();

            return Ok(());
        }

        if let Some((command, args)) = to_run.split_first() {
            std::process::Command::new(command)
                .args(args)
                .spawn()?
                .wait()?;
        }

        if null_terminate {
            print!("\0");
        }

        Ok(())
    }
}
