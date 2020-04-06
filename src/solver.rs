use crate::candidate_anagram::CandidateAnagram;
use crate::alphagram::Alphagram;
use crate::priority::Priority;
use priority_queue::PriorityQueue;

fn anagrams_for(user_input: &str) -> Vec<String> {
    let dict = vec!["fanhead", "car", "potatoes", "race", "floppy", "acre", "aa", "rcecr"];
    let requested_length = 2;

    let mut results = vec![];
    let mut pq = PriorityQueue::new();
    let c = CandidateAnagram{remaining_chars: Alphagram::new(user_input), next_word: 0};
    pq.push(c, Priority::new(vec![]));

    loop {
      let popped = pq.pop();
      if popped.is_none() {
          return results;
      }
      let (candidate, priority) = popped.unwrap();
      if candidate.is_complete() {
          results.push(priority_to_string(&priority, &dict));
          if results.len() >= requested_length {
              return results;
          }
          continue;
      }
      let word_index = candidate.next_word;
      let check_word = dict[word_index];
      let without_result = candidate.without(&Alphagram::new(check_word), word_index);
      without_result.and_then(|new_candidate| Ok(pq.push(new_candidate, priority.plus(word_index))));

      if candidate.next_word + 1 < dict.len() {
          let next_candidate = candidate.advanced_by(1);
          pq.push(next_candidate, priority);
      }
    }
}

fn priority_to_string(priority: &Priority, dict: &Vec<&str>) -> String {
    // priority.into_iter().map(|p| dict[p]).collect::<Vec<&str>>().join(&" ")
    priority.clone().into_iter().map(|p| dict[p]).collect::<Vec<&str>>().join(&" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::alphagram::Alphagram;
    use crate::priority::Priority;

    #[test]
    fn test_anagrams_for() {
        assert_eq!(anagrams_for("racecar"), vec!["car race", "car acre"]);
    }

    #[test]
    fn test_priority_to_string() {
        let dict = vec!["fanhead", "car", "potatoes", "race", "floppy", "acre", "aa", "rcecr"];
        let p = Priority::new(vec![1,3]);
        assert_eq!(priority_to_string(&p, &dict), "car race");
    }
}
