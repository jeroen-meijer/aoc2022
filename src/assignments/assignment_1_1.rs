use super::Assignment;

pub fn get_assignment() -> Assignment {
    return Assignment::new(1, 1, "TODO".to_string(), None, _run);
}

fn _run(_data: Vec<String>) -> Option<i32> {
    None
}
