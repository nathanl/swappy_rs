// A "candidate anagram" looks like: `CandidateAnagram{remaining_chars: Vec<Char>, priority: Priority{i: Vec<i32>}, next_candidate_word: usize}`
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
    pub next_word: usize
}

impl CandidateAnagram {
    pub fn new(phrase: &str) -> CandidateAnagram {
        CandidateAnagram {
            remaining_chars: Alphagram::new(&phrase),
            priority: Priority::new(vec![]),
            next_word: 0
        }
    }

    pub fn advanced_by(&self, count: usize) -> CandidateAnagram {
        CandidateAnagram{
            remaining_chars: self.remaining_chars.clone(),
            priority: self.priority.clone(),
            next_word: self.next_word + count
        }
    }

    pub fn without(&self, word: &Alphagram, index: usize) -> Result<CandidateAnagram, &'static str> {
        match self.remaining_chars.without(word) {
            Err(e) => Err(e),
            Ok(remainder) => {
                Ok(CandidateAnagram{
                    remaining_chars: remainder,
                    priority: self.priority.plus(index),
                    next_word: index
                })

            }
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

        #[test]
        fn test_without() {
            let original = CandidateAnagram{remaining_chars: Alphagram::new("race"), priority: Priority::new(vec![1]), next_word: 2};
            let without = original.without(&Alphagram::new("car"), 3).unwrap();
            let expected = CandidateAnagram{remaining_chars: Alphagram::new("e"), priority: Priority::new(vec![1, 3]), next_word: 3};
            assert_eq!(without, expected);

            let original = CandidateAnagram{remaining_chars: Alphagram::new("race"), priority: Priority::new(vec![1]), next_word: 2};
            let without = original.without(&Alphagram::new("bananar"), 3);
            // TODO assert Err without regard to actual message
            assert_eq!(without, Err("could not remove character"));
        }
    }
