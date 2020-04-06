// A "candidate anagram" looks like: `CandidateAnagram{remaining_chars: Vec<Char>, priority: Priority{i: Vec<i32>}, next_candidate_word: u32}`
// - Create the first candidate anagram and drop it in the priority queue. It looks like this:
//   - `priority` is `[]`
//   - `remaining_chars` has the full input alphagram
//   - `next_word` is `0`
use crate::alphagram::Alphagram;
use crate::priority::Priority;

#[derive(PartialEq, Eq, Debug)]
pub struct CandidateAnagram{
    pub remaining_chars: Alphagram,
    pub priority: Priority,
    pub next_word: u32
}

impl CandidateAnagram {
    pub fn new(phrase: &str) -> CandidateAnagram {
        CandidateAnagram {
            remaining_chars: Alphagram::new(&phrase),
            priority: Priority::new(vec![]),
            next_word: 0
        }
    }

    pub fn advanced_by(&self, count: u32) -> CandidateAnagram {
        CandidateAnagram{
            remaining_chars: self.remaining_chars.clone(),
            priority: self.priority.clone(),
            next_word: self.next_word + count
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_advanced_by() {
        let original = CandidateAnagram{remaining_chars: Alphagram::new(""), priority: Priority::new(vec![1,3]), next_word: 10};
        let advanced = original.advanced_by(11);
        let expected = CandidateAnagram{remaining_chars: Alphagram::new(""), priority: Priority::new(vec![1,3]), next_word: 21};
        assert_eq!(advanced, expected);
    }
}
