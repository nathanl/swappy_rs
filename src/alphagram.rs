use std::cmp::max;
use std::cmp::Ordering;

pub fn new(input: &str) -> Vec<char> {
    let mut chars: Vec<char> = input.to_lowercase().chars().collect();
    chars.sort();
    chars
}

#[derive(Debug)]
struct CandidatePriority {
    pub i: Vec<i32>,
}

impl CandidatePriority {
    fn new() -> CandidatePriority {
        let v: Vec<i32> = Vec::new();
        CandidatePriority { i: v }
    }

    fn append(&mut self, priority: i32) {
        self.i.push(priority);
    }
}

// impl Ord for CandidatePriority {
//     fn cmp(&self, other: &Self) -> Ordering {
//         self.height.cmp(&other.height)
//     }
// }

impl PartialEq for CandidatePriority {
    fn eq(&self, other: &Self) -> bool {
        self.i.eq(&other.i)
    }
}

impl Eq for CandidatePriority {}

impl Ord for CandidatePriority {
    // TODO refactor
    fn cmp(&self, other: &Self) -> Ordering {
        let self_is_shorter = self.i.len() < other.i.len();

        for pos in 0..max(self.i.len(), other.i.len()) {
            let first = self.i.get(pos);
            let second = other.i.get(pos);
            if first == None || second == None {
                return if self_is_shorter {
                    Ordering::Less
                } else {
                    Ordering::Greater
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
//
impl PartialOrd for CandidatePriority {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equal_to_self() {
        let mut c = CandidatePriority::new();
        // [] should equal []
        assert_eq!(c, c);
        c.append(1);
        // [1] should equal [1]
        assert_eq!(c, c);
    }

    #[test]
    fn order_by_index_or_length() {
        let mut c1 = CandidatePriority::new();
        c1.append(2);
        c1.append(2);
        let mut c2 = CandidatePriority::new();
        c2.append(2);
        c2.append(1);
        // [2,1] is greater than [2,2]
        assert!(c2 > c1);
        assert!(c1 < c2);

        let mut c1 = CandidatePriority::new();
        c1.append(1);
        let c2 = CandidatePriority::new();
        // [1] is greater than []
        assert!(c1 > c2);
        assert!(c2 < c1);

        let mut c2 = CandidatePriority::new();
        c2.append(2);
        c2.append(8);
        // [1] is greater than [2,8]
        assert!(c1 > c2);
        assert!(c2 < c1);
    }
}
