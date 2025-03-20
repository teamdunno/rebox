mod boxcmd;
mod registry;

use boxutils::commands::being_called_as;

fn main() {
    let utility_name = being_called_as();
    registry::get_registry().execute(&utility_name);
}
