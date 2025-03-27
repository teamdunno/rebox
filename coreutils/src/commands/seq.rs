use boxutils::{args::ArgParser, commands::Command};

pub struct Seq;

impl Command for Seq {
    fn execute(&self) {
        let args = ArgParser::builder()
            .add_option("-s")
            .add_flag("-w")
            .parse_args("seq");

        let pad_with_zeroes = args.get_flag("-w");
        let separator = args.get_option("-s").unwrap_or("\n");

        // note(teesh): I do not know why Rust considers this
        //              "unused", so I'm just gonna make it shut
        //              up about it haha
        #[allow(unused_assignments)]
        let (mut firstnum, mut incnum, mut lastnum) = (1, 1, 0);

        match &args.get_normal_args()[..] {
            [last] => {
                lastnum = last.parse().expect("seq: invalid last number");
            }
            [first, last] => {
                firstnum = first.parse().expect("seq: invalid first number");
                lastnum = last.parse().expect("seq: invalid last number");
            }
            [first, inc, last] => {
                firstnum = first.parse().expect("seq: invalid first number");
                incnum = inc.parse().expect("seq: invalid inc number");
                lastnum = last.parse().expect("seq: invalid last number");
            }
            _ => panic!("seq: malformed arguments"),
        }

        let mut accumulator = firstnum;

        // Find the width of the largest number
        let width = if pad_with_zeroes {
            // Calculate width based on the largest number in the sequence
            let max_value = if lastnum > firstnum {
                lastnum
            } else {
                firstnum
            };
            max_value.to_string().len()
        } else {
            0 // no padding if not using -w
        };

        while accumulator <= lastnum {
            // If padding is enabled, format the number with leading zeros
            if pad_with_zeroes {
                print!("{:0width$}", accumulator, width = width);
            } else {
                print!("{}", accumulator);
            }

            accumulator += incnum;
            if accumulator <= lastnum {
                print!("{}", separator);
            }
        }
    }
}
