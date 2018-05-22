pub fn number_of_lines(s: &str) -> usize {
    let v: Vec<&str> = s.split('\n').collect();
    match v.len() {
        0 => 0,
        _ => v.len() - 1,
    }
}

pub fn get_unfinished_lines(s: &str) -> usize {
    match s.rfind("\n") {
        Some(res) => s.len() - res - 1,
        None => s.len(),
    }
}
