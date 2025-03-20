mod boxcmd;
mod registry;

use std::env;
use std::path::Path;

fn being_called_as() -> String {
    let args = env::args().collect::<Vec<String>>();
    let exe_path = args[0].clone();
    let path = Path::new(&exe_path);
    let being_called = path.file_name().unwrap().to_str().unwrap().to_string();
    let formatted = being_called.replace(".exe", "");
    formatted
}

fn main() {
    let utility_name = being_called_as();
    registry::get_registry().execute(&utility_name);
}
