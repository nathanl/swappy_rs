#[derive(Debug)]
struct Alphagram(Vec<char>);

impl Alphagram {
    fn new(input: &str) -> Alphagram {
        let mut chars: Vec<char> = input.to_lowercase().chars().collect();
        chars.sort();
        Alphagram(chars)
    }
}

pub fn new(input: &str) -> Vec<char> {
    let mut chars: Vec<char> = input.to_lowercase().chars().collect();
    chars.sort();
    chars
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fooey() {
        let a = Alphagram::new("cake");
        println!("alphagram {:?}", a);
    }
}
