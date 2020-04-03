use std::collections::HashMap;

// https://doc.rust-lang.org/rust-by-example/generics/new_types.html
#[derive(Debug, PartialEq, Eq)]
struct Alphagram(HashMap<char, u8>);

impl Alphagram {
    pub fn new(input: &str) -> Alphagram {
        let lc = input.to_lowercase();
        let chars = lc.chars();
        let mut map: HashMap<char, u8> = HashMap::with_capacity(26);
        for this_char in chars {
            *map.entry(this_char).or_insert(0) += 1;
        }
        Alphagram(map)
    }

    pub fn to_hash(&self) -> &HashMap<char, u8> {
        &self.0
    }

    pub fn without(&self, needle: &Alphagram) -> Result<Alphagram, &'static str> {
        let mut remainder: HashMap<char, u8> = self.0.clone();
        for (&this_char, needle_count) in &needle.0 {
            let remainder_count = remainder.get(&this_char).unwrap_or(&0);
            if remainder_count < needle_count {
                return Err("could not remove character");
            } else {
                let new_remainder_count = remainder_count - needle_count;
                if new_remainder_count == 0 {
                    remainder.remove(&this_char);
                } else {
                    remainder.insert(this_char, new_remainder_count);
                }
            }
        }
        Ok(Alphagram(remainder))
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
    fn test_new() {
        let hm: HashMap<char, u8> = HashMap::new();
        assert_eq!(Alphagram::new("").to_hash(), &hm);

        let mut hm: HashMap<char, u8> = HashMap::new();
        hm.insert('a', 2);
        hm.insert('c', 2);
        hm.insert('r', 2);
        hm.insert('e', 1);
        assert_eq!(Alphagram::new("racecar").to_hash(), &hm);
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
