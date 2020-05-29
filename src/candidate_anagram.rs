use crate::alphagram::Alphagram;
use crate::priority::Priority;
// use crate::word::Word;

#[derive(PartialEq, Eq, Debug, Hash)]
pub struct CandidateAnagram {
    pub priority: Priority,
    pub remaining_chars: Alphagram,
    pub next_word: usize,
}

impl CandidateAnagram {
    pub fn new(phrase: &str) -> CandidateAnagram {
        CandidateAnagram {
            priority: Priority::new(vec![]),
            remaining_chars: Alphagram::new(&phrase),
            next_word: 0,
        }
    }

    pub fn advanced_by(&self, count: usize) -> CandidateAnagram {
        CandidateAnagram {
            priority: Priority::new(vec![]),
            remaining_chars: self.remaining_chars.clone(),
            next_word: self.next_word + count,
        }
    }

    pub fn without(
        &self,
        word: &Alphagram,
        index: usize,
    ) -> Result<CandidateAnagram, &'static str> {
        match self.remaining_chars.without(word) {
            Err(e) => Err(e),
            Ok(remainder) => Ok(CandidateAnagram {
                priority: self.priority.plus(index),
                remaining_chars: remainder,
                next_word: index,
            }),
        }
    }

    pub fn is_complete(&self) -> bool {
        self.remaining_chars.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_without() {
        let original = CandidateAnagram {
            priority: Priority::new(vec![]),
            remaining_chars: Alphagram::new("race"),
            next_word: 2,
        };
        let without = original.without(&Alphagram::new("car"), 3).unwrap();
        let expected = CandidateAnagram {
            priority: Priority::new(vec![3]),
            remaining_chars: Alphagram::new("e"),
            next_word: 3,
        };
        assert_eq!(without, expected);

        let original = CandidateAnagram {
            priority: Priority::new(vec![]),
            remaining_chars: Alphagram::new("race"),
            next_word: 2,
        };
        let without = original.without(&Alphagram::new("bananar"), 3);
        // TODO assert Err without regard to actual message
        assert_eq!(without, Err("needle not found"));
    }
}
