// https://doc.rust-lang.org/rust-by-example/generics/new_types.html
#[derive(Debug, Clone, Hash)]
pub struct Priority(Vec<usize>);

impl Priority {
    pub fn new(vec: Vec<usize>) -> Priority {
        Priority(vec)
    }

    pub fn plus(&self, i: usize) -> Priority {
        let mut new_vec = self.0.clone();
        new_vec.push(i);
        Priority(new_vec)
    }

    pub fn last(&self) -> Option<&usize> {
        self.0.last()
    }
}

impl IntoIterator for Priority {
    type Item = usize;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl PartialEq for Priority {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl Eq for Priority {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equal_to_self() {
        let empty = Priority::new(vec![]);
        assert_eq!(empty, empty);
        let one = Priority::new(vec![1]);
        assert_eq!(one, one);
    }

    #[test]
    fn test_push() {
        let empty = Priority::new(vec![]);
        let one = Priority::new(vec![1]);
        let appended = empty.plus(1);
        assert_eq!(one, appended);
    }
}
