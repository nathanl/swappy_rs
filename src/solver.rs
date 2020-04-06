use crate::candidate_anagram::CandidateAnagram;

// fn anagrams_for(user_input: &str) -> Vec<String> {
//     let _dict = vec!["fanhead", "car", "potatoes", "race", "floppy", "acre", "aa", "rcecr"];
//     let requested_length = 2;
//
//     let mut pq = PriorityQueue::new();
//     pq.push(CandidateAnagram(remaining_chars=Alphagram(user_input), priority=Priority(,), next_word=0));
//     loop {
//       let candidate: CandidateAnagram = match pq.pop {
//           Some(thing) => thing,
//           None => return results;
//       }
//       if candidate.is_complete() {
//           results.append(candidate.to_string());
//           if results.len() >= requested_length) {
//               return results;
//           }
//           continue;
//       }
//       if candidate.next_word + 1 < dict.len() {
//           pq.push(candidate.advanced_by(1));
//       }
//       let check_word = dict[candidate.word_to_check];
//       let new_candidate: Option<CandidateAnagram> = candidate.try_this_one(check_word);
//       if new_candidate is not none {
//           pq.push(new_candidate);
//       }
//     }
//
// }

fn candidate_to_string(c: CandidateAnagram, dict: Vec<&str>) -> String {
    c.priority.into_iter().map(|p| dict[p]).collect::<Vec<&str>>().join(&" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::alphagram::Alphagram;
    use crate::priority::Priority;

    // #[test]
    // fn test_anagrams_for() {
    //     assert_eq!(anagrams_for("racecar"), vec!["car race", "car acre"]);
    // }

    #[test]
    fn test_candidate_to_string() {
        let dict = vec!["fanhead", "car", "potatoes", "race", "floppy", "acre", "aa", "rcecr"];
        let c = CandidateAnagram{remaining_chars: Alphagram::new(""), priority: Priority::new(vec![1,3]), next_word: 10};
        assert_eq!(candidate_to_string(c, dict), "car race");
    }
}
