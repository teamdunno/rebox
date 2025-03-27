// this function makes it so that an EOF (commonly used in pipes) stops the repl
pub fn get_like_repl(buffer: &mut String) -> bool {
    std::io::stdin().read_line(buffer).unwrap_or(0) > 0
}
