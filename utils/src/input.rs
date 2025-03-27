use std::io::Read;

// this function makes it so that an EOF (commonly used in pipes) stops the repl
pub fn repl(buffer: &mut String) -> bool {
    std::io::stdin().read_line(buffer).unwrap_or(0) > 0
}

pub fn repl_with_file(file: &mut Box<dyn Read>, buffer: &mut String) -> bool {
    file.read_to_string(buffer).unwrap_or(0) > 0
}