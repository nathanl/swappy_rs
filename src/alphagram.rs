pub fn new(input: &str) -> Vec<char> {
    let mut chars: Vec<char> = input
        .to_lowercase()
        .chars()
        .collect();
    chars.sort();
    chars
}
