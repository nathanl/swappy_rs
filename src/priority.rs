use std::cmp::max;
use std::cmp::Ordering;

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

impl Ord for Priority {
    // TODO refactor
    fn cmp(&self, other: &Self) -> Ordering {
        let self_is_shorter = self.0.len() < other.0.len();

        for pos in 0..max(self.0.len(), other.0.len()) {
            let first = self.0.get(pos);
            let second = other.0.get(pos);
            if first == None || second == None {
                return if self_is_shorter {
                    Ordering::Less // deepest wins
                                   // Ordering::Greater // shallowest wins
                } else {
                    Ordering::Greater // deepest wins
                                      // Ordering::Less // shallowest wins
                };
            }
            if first == second {
                continue;
            } else if first < second {
                return Ordering::Greater;
            } else {
                return Ordering::Less;
            }
        }
        Ordering::Equal
    }
}

impl PartialOrd for Priority {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use priority_queue::PriorityQueue;
    // use keyed_priority_queue::KeyedPriorityQueue as PriorityQueue;

    #[test]
    fn equal_to_self() {
        let empty = Priority::new(vec![]);
        assert_eq!(empty, empty);
        let one = Priority::new(vec![1]);
        assert_eq!(one, one);
    }

    #[test]
    fn order_by_index_or_length() {
        let two_two = Priority::new(vec![2, 2]);
        let two_one = Priority::new(vec![2, 1]);
        assert!(two_one > two_two);
        assert!(two_two < two_one);

        let one = Priority::new(vec![1]);
        let empty = Priority::new(vec![]);
        assert!(one > empty);
        assert!(empty < one);

        let two_eight = Priority::new(vec![2, 8]);
        assert!(one > two_eight);
        assert!(two_eight < one);
    }

    #[test]
    fn test_push() {
        let empty = Priority::new(vec![]);
        let one = Priority::new(vec![1]);
        let appended = empty.plus(1);
        assert_eq!(one, appended);
    }

    #[test]
    fn priorites_work_in_pq() {
        let two_two = Priority::new(vec![2, 2]);

        let one = Priority::new(vec![1]);

        // let mut pq = PriorityQueue::new();
        let mut pq = PriorityQueue::new();
        pq.push("Apples", &two_two);
        pq.push("Bananas", &one);
        let res = pq.pop();
        assert_eq!(res, Some(("Bananas", &one)))
    }
}
