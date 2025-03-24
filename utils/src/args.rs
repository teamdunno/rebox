use std::collections::{HashMap, HashSet};

use crate::commands::get_args;

pub struct ArgParser {
    #[allow(dead_code)]
    flags: HashSet<String>,
    #[allow(dead_code)]
    options: HashSet<String>,
    parsed_flags: HashSet<String>,
    parsed_options: HashMap<String, String>,
    normal_args: Vec<String>,
}

impl ArgParser {
    pub fn builder() -> ArgParserBuilder {
        ArgParserBuilder::new()
    }

    pub fn get_flag(&self, flag: &str) -> bool {
        self.parsed_flags.contains(flag)
    }

    pub fn get_option(&self, option: &str) -> Option<&str> {
        self.parsed_options.get(option).map(String::as_str)
    }

    pub fn get_normal_args(&self) -> Vec<String> {
        self.normal_args.clone()
    }
}

pub struct ArgParserBuilder {
    flags: HashSet<String>,
    options: HashSet<String>,
}

impl ArgParserBuilder {
    pub fn new() -> Self {
        Self {
            flags: HashSet::new(),
            options: HashSet::new(),
        }
    }

    pub fn add_flag(mut self, flag: &str) -> Self {
        self.flags.insert(flag.to_string());
        self
    }

    pub fn add_flags(mut self, flags: Vec<&str>) -> Self {
        for flag in flags {
            self.flags.insert(flag.to_string());
        }
        self
    }

    pub fn add_option(mut self, option: &str) -> Self {
        self.options.insert(option.to_string());
        self
    }

    pub fn add_options(mut self, options: Vec<&str>) -> Self {
        for option in options {
            self.options.insert(option.to_string());
        }
        self
    }

    pub fn parse(self, program_name: &str, args: Vec<String>) -> ArgParser {
        let args = get_args(program_name.to_string(), args);

        let mut parsed_flags = HashSet::new();
        let mut parsed_options = HashMap::new();
        let mut normal_args = Vec::new();

        let mut iter = args.into_iter();
        while let Some(arg) = iter.next() {
            if self.flags.contains(&arg) {
                parsed_flags.insert(arg.clone());
            } else if self.options.contains(&arg) {
                if let Some(value) = iter.next() {
                    parsed_options.insert(arg.clone(), value);
                }
            } else if arg.len() > 2 && arg.starts_with('-') && !arg.starts_with("--") {
                let mut chars = arg.chars().skip(1).peekable();
                while let Some(ch) = chars.next() {
                    let flag = format!("-{}", ch);
                    if self.flags.contains(&flag) {
                        parsed_flags.insert(flag);
                    } else if self.options.contains(&flag) {
                        let value: String = chars.collect();
                        if !value.is_empty() {
                            parsed_options.insert(flag, value);
                        } else if let Some(next_value) = iter.next() {
                            parsed_options.insert(flag, next_value);
                        }
                        break;
                    }
                }
            } else {
                normal_args.push(arg);
            }
        }

        ArgParser {
            flags: self.flags,
            options: self.options,
            parsed_flags,
            parsed_options,
            normal_args,
        }
    }

    pub fn parse_args(self, program_name: &str) -> ArgParser {
        self.parse(program_name, std::env::args().collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_flag() {
        let args = ArgParser::builder()
            .add_flag("-r")
            .parse("sometool", vec!["sometool".to_string(), "-r".to_string()]);
        assert!(args.get_flag("-r"));
    }

    #[test]
    fn test_multiple_flags() {
        let args = ArgParser::builder()
            .add_flag("-r")
            .add_flag("-f")
            .add_flag("-v")
            .parse("sometool", vec!["sometool".to_string(), "-rfv".to_string()]);
        assert!(args.get_flag("-r"));
        assert!(args.get_flag("-f"));
        assert!(args.get_flag("-v"));
    }

    #[test]
    fn test_single_option() {
        let args = ArgParser::builder()
            .add_option("-O")
            .parse("sometool", vec![
                "sometool".to_string(),
                "-O".to_string(),
                "fast".to_string(),
            ]);
        assert_eq!(args.get_option("-O"), Some("fast"));
    }

    #[test]
    fn test_multiple_options() {
        let args = ArgParser::builder()
            .add_option("-O")
            .add_option("-hello")
            .parse("sometool", vec![
                "sometool".to_string(),
                "-O".to_string(),
                "fast".to_string(),
                "-hello".to_string(),
                "world".to_string(),
            ]);
        assert_eq!(args.get_option("-O"), Some("fast"));
        assert_eq!(args.get_option("-hello"), Some("world"));
    }

    #[test]
    fn test_all_together_now() {
        let args = ArgParser::builder()
            .add_flags(vec!["-r", "-f", "-v", "--do-it"])
            .add_options(vec!["-O", "-hello", "--another"])
            .parse("sometool", vec![
                "sometool".to_string(),
                "-rf".to_string(),
                "-v".to_string(),
                "-Ofast".to_string(),
                "-hello".to_string(),
                "world".to_string(),
                "--do-it".to_string(),
                "somefile.txt".to_string(),
                "--another".to_string(),
                "file.txt".to_string(),
            ]);
        assert!(args.get_flag("-r"));
        assert!(args.get_flag("-f"));
        assert!(!args.get_flag("-g"));
        assert!(args.get_flag("-v"));
        assert!(args.get_flag("--do-it"));
        assert_eq!(args.get_option("-O"), Some("fast"));
        assert_eq!(args.get_option("-hello"), Some("world"));
        assert_eq!(args.get_option("--another"), Some("file.txt"));
        assert_eq!(args.get_option("--does-not-exist"), None);
        assert_eq!(args.get_normal_args(), vec!["somefile.txt".to_string()]);
    }
}
