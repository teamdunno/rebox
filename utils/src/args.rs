use super::commands::get_args;

pub struct Args {
    args: Vec<String>,
}

impl Args {
    pub fn new(command: &str, args: Vec<String>) -> Args {
        let args = get_args(command.into(), args);

        Args { args }
    }

    pub fn get_flag(&self, flag: &str) -> bool {
        self.args
            .iter()
            .any(|arg| arg == flag || arg.starts_with(flag))
    }

    pub fn get_args(&self) -> Vec<String> {
        self.args.clone()
    }

    pub fn get_option(&self, option: &str) -> Option<String> {
        // Check for the option as a separate argument
        if let Some(i) = self.args.iter().position(|x| x == option) {
            if i + 1 < self.args.len() {
                return Some(self.args[i + 1].clone());
            }
        }

        // Check for the option as a combined argument (-fvalue)
        for arg in &self.args {
            if arg.starts_with(option) && arg.len() > option.len() {
                return Some(arg[option.len()..].to_string());
            }
        }

        None
    }
}

mod tests {
    #[test]
    fn test_flag() {
        let args = super::Args::new("sometool", vec!["sometool".into(), "-f".into()]);
        assert_eq!(args.get_flag("-f"), true);
    }

    #[test]
    fn test_no_flag() {
        let args = super::Args::new("sometool", vec!["sometool".into(), "-f".into()]);
        assert_eq!(args.get_flag("-g"), false);
    }

    #[test]
    fn test_option() {
        let args = super::Args::new("sometool", vec![
            "sometool".into(),
            "-f".into(),
            "value".into(),
        ]);
        assert_eq!(args.get_option("-f"), Some("value".into()));
    }

    #[test]
    fn test_no_option() {
        let args = super::Args::new("sometool", vec!["sometool".into(), "-f".into()]);
        assert_eq!(args.get_option("-g"), None);
    }

    #[test]
    fn test_combined_option() {
        let args = super::Args::new("sometool", vec!["sometool".into(), "-fvalue".into()]);
        assert_eq!(args.get_option("-f"), Some("value".into()));
    }

    #[test]
    fn test_args() {
        let args = super::Args::new("sometool", vec!["sometool".into(), "-f".into()]);
        assert_eq!(args.get_args(), vec!["-f".to_string()]);
    }
}
