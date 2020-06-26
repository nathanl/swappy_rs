use crate::alphagram::Alphagram;
use crate::found_words::FoundWords;

// Represents the words found so far in an input phrase and the remaining characters in which we
// could find words. Each CandidateAnagram is a node in our search tree.
// A CandidateAnagram's children (if any) are all the CandidateAnagrams we can build by removing
// another word.
// A CandidateAnagram is complete if there are no remaining characters - that would make it a
// "success" leaf node.
// Many CandidateAnagrams are not complete and also have no children; eg, a CandidateAnagram
// which has only the letter "z" in remaining_chars isn't complete, but no more words can be found.
// In that case, it's a "dead end" leaf node in our tree.
#[derive(PartialEq, Eq, Debug)]
pub struct CandidateAnagram {
    pub words_found: FoundWords,
    pub remaining_chars: Alphagram,
}

impl CandidateAnagram {
    pub fn new(phrase: &str) -> CandidateAnagram {
        CandidateAnagram {
            words_found: FoundWords::new(vec![]),
            remaining_chars: Alphagram::new(&phrase),
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
                words_found: self.words_found.plus(index),
                remaining_chars: remainder,
            }),
        }
    }

    pub fn is_complete(&self) -> bool {
        self.remaining_chars.is_empty()
    }

    pub fn children(&self, word_list: &Vec<(&String, Alphagram)>) -> Vec<CandidateAnagram> {
        let mut children: Vec<CandidateAnagram> = vec![];
        let p = self.words_found.last().unwrap_or(&0usize).clone();

        for i in p..word_list.len() {
            let alphagram = &word_list[i].1;

            match self.without(alphagram, i) {
                Err(_e) => continue,
                Ok(new_candidate) => {
                    children.push(new_candidate);
                }
            }
        }
        return children;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_without() {
        let original = CandidateAnagram {
            words_found: FoundWords::new(vec![]),
            remaining_chars: Alphagram::new("race"),
        };
        let without = original.without(&Alphagram::new("car"), 3).unwrap();
        let expected = CandidateAnagram {
            words_found: FoundWords::new(vec![3]),
            remaining_chars: Alphagram::new("e"),
        };
        assert_eq!(without, expected);

        let original = CandidateAnagram {
            words_found: FoundWords::new(vec![]),
            remaining_chars: Alphagram::new("race"),
        };
        let without = original.without(&Alphagram::new("bananar"), 3);
        // TODO assert Err without regard to actual message
        assert_eq!(without, Err("needle not found"));
    }
}
