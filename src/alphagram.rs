use std::collections::HashMap;
use std::hash::{Hash, Hasher};

// https://doc.rust-lang.org/rust-by-example/generics/new_types.html
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Alphagram(HashMap<char, u8>, u8);

// NOTE: the priority queue calls this function, but apparently the
// function always returns ()...
// ...and that doesn't matter? :`¯\_(ツ)_/¯`:
impl Hash for Alphagram {
    fn hash<H: Hasher>(&self, _state: &mut H) {
        ()
        // let str = format!("{:?}", self.0);
        // str.hash(state);
    }
}

impl Alphagram {
    pub fn new(input: &str) -> Alphagram {
        let lc = input.to_lowercase();
        // TODO - more robust filtering of whitespace
        // (tried with https://crates.io/crates/regex and \W but it was way too slow)
        let chars = lc.chars().filter(|c| c != &' ');
        let mut map: HashMap<char, u8> = HashMap::with_capacity(26);
        let mut length: u8 = 0;
        for this_char in chars {
            *map.entry(this_char).or_insert(0) += 1;
            length +=1;
        }
        Alphagram(map, length)
    }

    fn unique_char_count(&self) -> usize {
        self.0.len()
    }

    pub fn without(&self, needle: &Alphagram) -> Result<Alphagram, &'static str> {
        if needle.1 > self.1 {
            return Err("needle is shorter than haystack");
        }

        if needle.unique_char_count() > self.unique_char_count() {
            return Err("some letters in needle are not in haystack");
        }

        let mut haystack: HashMap<char, u8> = self.0.clone();
        for (&this_char, needle_count) in &needle.0 {
            let haystack_count = haystack.get(&this_char).unwrap_or(&0);
            if haystack_count < needle_count {
                return Err("could not remove character");
            } else {
                let new_haystack_count = haystack_count - needle_count;
                if new_haystack_count == 0 {
                    haystack.remove(&this_char);
                } else {
                    haystack.insert(this_char, new_haystack_count);
                }
            }
        }
        Ok(Alphagram(haystack, self.1 - needle.1))
    }

    pub fn contains(&self, other: &Alphagram) -> bool {
        match self.without(other) {
            Ok(_) => true,
            _ => false,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.0.len() == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_leftover(haystack: &str, needle: &str, expected: &str) {
        let haystack = Alphagram::new(haystack);
        let needle = Alphagram::new(needle);
        let remaining = haystack.without(&needle).unwrap(); // Ok(Alphagram) or Err("not in there")
        assert_eq!(remaining, Alphagram::new(expected));
    }

    fn refute_contains(haystack: &str, needle: &str) {
        let haystack = Alphagram::new(haystack);
        let needle = Alphagram::new(needle);
        let remaining = haystack.without(&needle); // Ok(Alphagram) or Err("not in there")
        match remaining {
            Err(_) => (),
            Ok(_) => panic!("expected this to not work"),
        }
    }

    #[test]
    fn ignores_whitespace() {
        let ag1 = Alphagram::new("racecar");
        let ag2 = Alphagram::new("race car");
        assert_eq!(ag1, ag2);
    }

    #[test]
    fn removals() {
        let haystack = Alphagram::new("racecar");
        let needle = Alphagram::new("car");
        let remaining = haystack.without(&needle).unwrap();
        assert_eq!(remaining, Alphagram::new("race"));

        assert_leftover("racecar", "car", "race");
        assert_leftover("racecar", "race", "car");
        assert_leftover("racecar", "racecar", "");
        assert_leftover("", "", "");
        refute_contains("racecar", "banana");
        refute_contains("", "car");
    }
}
