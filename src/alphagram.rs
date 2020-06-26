use std::collections::HashMap;

/// Represents the "bag" or "multiset" (non-unique set) of characters contained in a word or phrase.
/// Eg, "bat" and "tab" have the same alphagram, but "tint" is not the same alphagram as "tin".
/// Here we represent the alphagram as a hashmap, where the keys are characters and the values are
/// the number of times they occur. This allows for efficient subtraction.
// https://doc.rust-lang.org/rust-by-example/generics/new_types.html
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Alphagram(HashMap<char, u8>, u8);

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
            length += 1;
        }
        Alphagram(map, length)
    }

    fn unique_char_count(&self) -> usize {
        self.0.len()
    }

    pub fn without(&self, needle: &Alphagram) -> Result<Alphagram, &'static str> {
        if !self.contains(needle) {
            return Err("needle not found");
        }

        // Now that we know we can remove needle from haystack, actually do so
        let mut haystack: HashMap<char, u8> = self.0.clone();
        for (&this_char, needle_count) in &needle.0 {
            let haystack_count = haystack.get(&this_char).unwrap_or(&0);
            let new_haystack_count = haystack_count - needle_count;
            if new_haystack_count == 0 {
                haystack.remove(&this_char);
            } else {
                haystack.insert(this_char, new_haystack_count);
            }
        }
        Ok(Alphagram(haystack, self.1 - needle.1))
    }

    // This is the "hot path" - the vast majority of the time we will find that we can't remove one
    // alphagram from another, and we want to determine that as quickly as possible
    // NOTE: could we speed this up with an XOR of bitsets? XOR of the two + AND with the needle
    // where result is non-zero? How would this work with unicode?
    pub fn contains(&self, needle: &Alphagram) -> bool {
        if needle.1 > self.1 {
            return false; // needle is shorter than haystack
        }

        if needle.unique_char_count() > self.unique_char_count() {
            return false; // some letters in needle are not in haystack
        }

        for (&this_char, needle_count) in &needle.0 {
            if self.0.get(&this_char).unwrap_or(&0) < needle_count {
                return false; // haystack doesn't have enough of this letter
            }
        }
        true
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
