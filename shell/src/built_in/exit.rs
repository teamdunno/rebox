use crate::built_in::Action;

pub fn exit(_: Vec<&str>) -> Action {
    Action::Exit
}
