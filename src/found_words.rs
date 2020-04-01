// Represents a path through the tree of possible anagrams.
// Eg, if we have found words in our input phrase corresponding to items
// 1, 3, and 8 in the wordlist, we'd have FoundWords::new(vec![1, 3, 8])
// https://doc.rust-lang.org/rust-by-example/generics/new_types.html
#[derive(Debug, Clone, Hash)]
pub struct FoundWords(Vec<usize>);

impl FoundWords {
    pub fn new(vec: Vec<usize>) -> FoundWords {
        FoundWords(vec)
    }

    pub fn plus(&self, i: usize) -> FoundWords {
        let mut new_vec = self.0.clone();
        new_vec.push(i);
        FoundWords(new_vec)
    }

    pub fn last(&self) -> Option<&usize> {
        self.0.last()
    }
}

impl IntoIterator for FoundWords {
    type Item = usize;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl PartialEq for FoundWords {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl Eq for FoundWords {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equal_to_self() {
        let empty = FoundWords::new(vec![]);
        assert_eq!(empty, empty);
        let one = FoundWords::new(vec![1]);
        assert_eq!(one, one);
    }

    #[test]
    fn test_push() {
        let empty = FoundWords::new(vec![]);
        let one = FoundWords::new(vec![1]);
        let appended = empty.plus(1);
        assert_eq!(one, appended);
    }
}
