use std::cmp::Ordering;

pub fn new(input: &str) -> Vec<char> {
    let mut chars: Vec<char> = input
        .to_lowercase()
        .chars()
        .collect();
    chars.sort();
    chars
}

#[derive(Debug, PartialEq)]
struct CandidatePriority {
    pub i: Vec<i32>
}

impl CandidatePriority {
    fn new() -> CandidatePriority {
        let v: Vec<i32> = Vec::new();
        CandidatePriority{i: v}
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

// impl PartialOrd for CandidatePriority {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         if self.i.len == other.i.len {
//
//         } else {
//             self.i.len.partial_cmp(other.i.len)
//         }
//
//         // Some(self.cmp(other))
//     }
// }
//
// impl PartialEq for CandidatePriority {
//     fn eq(&self, other: &Self) -> bool {
//         self.i == other.i
//     }
// }

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
    fn order_by_length() {
        let mut c1 = CandidatePriority::new();
        c1.append(1);
        let c2 = CandidatePriority::new();
        // [1] should come before []
        assert!(c1 > c2);
    }

    #[test]
    fn order_by_index() {
        let mut c1 = CandidatePriority::new();
        c1.append(2);
        c1.append(2);
        let mut c2 = CandidatePriority::new();
        c2.append(2);
        c2.append(1);
        // [2,1] should come before [2,2]
        assert!(c2 > c1);
    }
}
