use boxutils::args::ArgParser;
use boxutils::commands::Command;

// TODO: Add filetype operations
//
// This is not done yet because file operations are
// quite tricky to implement in a way that is both
// platform-independent and safe.

const STRING_TESTS: &[&str] = &["-z", "-n", "=", "!="];
const NUMBER_TESTS: &[&str] = &["-eq", "-ne", "-lt", "-le", "-gt", "-ge"];
const OTHER_FLAGS: &[&str] = &["]"];

fn exit_false() {
    std::process::exit(1);
}

fn exit_true() {
    std::process::exit(0);
}

pub struct Test {
    bracket: bool,
}

impl Test {
    pub fn with_bracket() -> Test {
        Test { bracket: true }
    }

    pub fn without_bracket() -> Test {
        Test { bracket: false }
    }

    fn do_string_ops(args: &ArgParser) {
        if args.get_flag("-z") {
            let args = args.get_normal_args();
            let string = args.get(0).unwrap_or(&String::from("")).clone();
            if string.is_empty() {
                exit_true();
            } else {
                exit_false();
            }
        }

        if args.get_flag("-n") {
            let args = args.get_normal_args();
            let string = args.get(0).unwrap_or(&String::from("")).clone();
            if !string.is_empty() {
                exit_true();
            } else {
                exit_false();
            }
        }

        if args.get_flag("=") {
            let args = args.get_normal_args();
            let string1 = args.get(0).unwrap_or(&String::from("")).clone();
            let string2 = args.get(1).unwrap_or(&String::from("")).clone();
            if string1 == string2 {
                exit_true();
            } else {
                exit_false();
            }
        }

        if args.get_flag("!=") {
            let args = args.get_normal_args();
            let string1 = args.get(0).unwrap_or(&String::from("")).clone();
            let string2 = args.get(1).unwrap_or(&String::from("")).clone();
            if string1 != string2 {
                exit_true();
            } else {
                exit_false();
            }
        }
    }

    fn do_number_ops(args: &ArgParser) {
        if args.get_flag("-eq") {
            let args = args.get_normal_args();
            let number1 = args
                .get(0)
                .unwrap_or(&String::from(""))
                .parse::<i64>()
                .unwrap_or(0);
            let number2 = args
                .get(1)
                .unwrap_or(&String::from(""))
                .parse::<i64>()
                .unwrap_or(0);
            if number1 == number2 {
                exit_true();
            } else {
                exit_false();
            }
        }

        if args.get_flag("-ne") {
            let args = args.get_normal_args();
            let number1 = args
                .get(0)
                .unwrap_or(&String::from(""))
                .parse::<i64>()
                .unwrap_or(0);
            let number2 = args
                .get(1)
                .unwrap_or(&String::from(""))
                .parse::<i64>()
                .unwrap_or(0);
            if number1 != number2 {
                exit_true();
            } else {
                exit_false();
            }
        }

        if args.get_flag("-lt") {
            let args = args.get_normal_args();
            let number1 = args
                .get(0)
                .unwrap_or(&String::from(""))
                .parse::<i64>()
                .unwrap_or(0);
            let number2 = args
                .get(1)
                .unwrap_or(&String::from(""))
                .parse::<i64>()
                .unwrap_or(0);
            if number1 < number2 {
                exit_true();
            } else {
                exit_false();
            }
        }

        if args.get_flag("-le") {
            let args = args.get_normal_args();
            let number1 = args
                .get(0)
                .unwrap_or(&String::from(""))
                .parse::<i64>()
                .unwrap_or(0);
            let number2 = args
                .get(1)
                .unwrap_or(&String::from(""))
                .parse::<i64>()
                .unwrap_or(0);
            if number1 <= number2 {
                exit_true();
            } else {
                exit_false();
            }
        }

        if args.get_flag("-gt") {
            let args = args.get_normal_args();
            let number1 = args
                .get(0)
                .unwrap_or(&String::from(""))
                .parse::<i64>()
                .unwrap_or(0);
            let number2 = args
                .get(1)
                .unwrap_or(&String::from(""))
                .parse::<i64>()
                .unwrap_or(0);
            if number1 > number2 {
                exit_true();
            } else {
                exit_false();
            }
        }

        if args.get_flag("-ge") {
            let args = args.get_normal_args();
            let number1 = args
                .get(0)
                .unwrap_or(&String::from(""))
                .parse::<i64>()
                .unwrap_or(0);
            let number2 = args
                .get(1)
                .unwrap_or(&String::from(""))
                .parse::<i64>()
                .unwrap_or(0);
            if number1 >= number2 {
                exit_true();
            } else {
                exit_false();
            }
        }
    }
}

impl Command for Test {
    fn execute(&self) {
        let args = ArgParser::builder()
            .add_flags(STRING_TESTS.into())
            .add_flags(NUMBER_TESTS.into())
            .add_flags(OTHER_FLAGS.into())
            .parse_args(if self.bracket { "[" } else { "test" });

        if self.bracket && !args.get_flag("]") {
            panic!("[: missing ]");
        }

        if STRING_TESTS.iter().any(|&x| args.get_flag(x)) {
            Test::do_string_ops(&args);
        }

        if NUMBER_TESTS.iter().any(|&x| args.get_flag(x)) {
            Test::do_number_ops(&args);
        }

        exit_false();
    }
}
