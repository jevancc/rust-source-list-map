pub fn number_of_lines(s: &str) -> usize {
    s.split('\n').count().saturating_sub(1)
}

pub fn get_unfinished_lines(s: &str) -> usize {
    match s.rfind('\n') {
        Some(res) => s.len() - res - 1,
        None => s.len(),
    }
}
