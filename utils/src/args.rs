use std::collections::{HashMap, HashSet};

use crate::commands::get_args;

pub struct ArgParser {
    #[allow(dead_code)]
    flags: HashSet<String>,
    #[allow(dead_code)]
    options: HashSet<String>,
    #[allow(dead_code)]
    option_lists: HashSet<String>,
    parsed_flags: HashSet<String>,
    parsed_options: HashMap<String, String>,
    parsed_option_lists: HashMap<String, Vec<String>>,
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

    pub fn get_option_list(&self, option: &str) -> Option<Vec<&str>> {
        self.parsed_option_lists
            .get(option)
            .map(|a| a.iter().map(String::as_str).collect::<Vec<&str>>())
    }

    pub fn get_normal_args(&self) -> Vec<String> {
        self.normal_args.clone()
    }
}

pub struct ArgParserBuilder {
    flags: HashSet<String>,
    options: HashSet<String>,
    option_lists: HashSet<String>,
    seperator: String,
}

impl ArgParserBuilder {
    pub fn new() -> Self {
        Self {
            flags: HashSet::new(),
            options: HashSet::new(),
            option_lists: HashSet::new(),
            seperator: "".into(),
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

    pub fn add_option_list(mut self, option_list: &str) -> Self {
        self.option_lists.insert(option_list.to_string());
        self
    }

    pub fn add_option_lists(mut self, option_lists: Vec<&str>) -> Self {
        for option in option_lists {
            self.option_lists.insert(option.to_string());
        }
        self
    }

    pub fn set_seperator(mut self, seperator: &str) -> Self {
        self.seperator = seperator.into();
        self
    }

    pub fn parse(self, program_name: &str, args: Vec<String>) -> ArgParser {
        let args = get_args(program_name.to_string(), args);

        let mut parsed_flags = HashSet::new();
        let mut parsed_options = HashMap::new();
        let mut parsed_option_lists: HashMap<String, Vec<String>> = HashMap::new();
        let mut normal_args = Vec::new();

        let mut iter = args.into_iter();
        while let Some(arg) = iter.next() {
            if arg == self.seperator {
                break;
            }

            if self.flags.contains(&arg) {
                parsed_flags.insert(arg.clone());
            } else if self.options.contains(&arg) {
                if let Some(value) = iter.next() {
                    parsed_options.insert(arg.clone(), value);
                }
            } else if self.option_lists.contains(&arg) {
                if let Some(value) = iter.next() {
                    // Corrected: use `arg` instead of `flag` for lookup and insertion
                    parsed_option_lists
                        .entry(arg.clone()) // Use arg.clone() to insert into the map
                        .or_insert_with(Vec::new) // Insert a new vector if none exists
                        .push(value); // Push the new value to the list
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
                    } else if self.option_lists.contains(&flag) {
                        let value: String = chars.clone().collect();
                        if !value.is_empty() {
                            parsed_option_lists
                                .entry(flag.clone()) // Use flag.clone() for the option list key
                                .or_insert_with(Vec::new)
                                .push(value);
                        } else if let Some(next_value) = iter.next() {
                            parsed_option_lists
                                .entry(flag.clone())
                                .or_insert_with(Vec::new)
                                .push(next_value);
                        }
                    }
                }
            } else {
                normal_args.push(arg);
            }
        }

        while let Some(arg) = iter.next() {
            normal_args.push(arg.to_string());
        }

        ArgParser {
            flags: self.flags,
            options: self.options,
            option_lists: self.option_lists,
            parsed_flags,
            parsed_options,
            normal_args,
            parsed_option_lists,
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

    #[test]
    fn test_option_lists() {
        let args = ArgParser::builder()
            .add_option_list("-u")
            .parse("sometool", vec![
                "sometool".into(),
                "-u".into(),
                "PATH".into(),
                "-uWSL_ENV".into(),
            ]);

        assert_eq!(args.get_option_list("-u"), Some(vec!["PATH", "WSL_ENV"]));
    }

    #[test]
    fn test_seperated() {
        let args = ArgParser::builder()
            .add_option_list("-u")
            .add_flag("--someflag")
            .set_seperator("--")
            .parse("sometool", vec![
                "sometool".into(),
                "-u".into(),
                "PATH".into(),
                "--".into(),
                "cargo".into(),
                "--someflag".into(),
                "-uIDK".into(),
            ]);

        assert_eq!(args.get_option_list("-u"), Some(vec!["PATH"]));
        assert_eq!(args.get_flag("--someflag"), false);
        assert_eq!(args.get_normal_args(), vec!["cargo", "--someflag", "-uIDK"]);
    }
}
