use crate::candidate_anagram::CandidateAnagram;
use crate::alphagram::Alphagram;
use crate::priority::Priority;
use priority_queue::PriorityQueue;

pub fn anagrams_for(user_input: &str, dict: &Vec<String>, requested_length: usize) -> Vec<String> {
    let mut results = vec![];
    println!("Prepping the dictionary");
    let dict: Vec<_> = dict.iter().map(|word| (word, Alphagram::new(word))).collect();
    println!("Prepped the dictionary");
    let mut pq = PriorityQueue::new();
    let c = CandidateAnagram::new(user_input);
    pq.push(c, Priority::new(vec![]));

    loop {
      let popped = pq.pop();
      if popped.is_none() {
          return results;
      }
      let (candidate, priority) = popped.unwrap();
      if candidate.is_complete() {
          let result_string = priority_to_string(&priority, &dict);
          results.push(result_string);
          if results.len() >= requested_length {
              return results;
          }
          continue;
      }
      let word_index = candidate.next_word;
      let dictionary_alphagram = &dict[word_index].1;
      let without_result = candidate.without(&dictionary_alphagram, word_index);
      without_result.and_then(|new_candidate| Ok(pq.push(new_candidate, priority.plus(word_index))));

      if candidate.next_word + 1 < dict.len() {
          let next_candidate = candidate.advanced_by(1);
          pq.push(next_candidate, priority);
      }
    }
}

fn priority_to_string(priority: &Priority, dict: &Vec<(&String,Alphagram)>) -> String {
    let mut result = String::new();
    for number in priority.clone().into_iter() {
        result.push_str(&dict[number].0.clone());
        result.push_str(" ");
    }
    return result;
    //priority.clone().into_iter().map(|p| dict[p]).clone().collect::<Vec<String>>().join(&" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::alphagram::Alphagram;
    use crate::priority::Priority;

    #[test]
    fn test_anagrams_for() {
        let dict = vec!["fanhead", "car", "potatoes", "race", "floppy", "acre", "aa", "rcecr"];
        assert_eq!(anagrams_for("racecar", &dict), vec!["car race", "car acre"]);
    }

    #[test]
    fn test_priority_to_string() {
        let dict = vec!["fanhead", "car", "potatoes", "race", "floppy", "acre", "aa", "rcecr"];
        let p = Priority::new(vec![1,3]);
        assert_eq!(priority_to_string(&p, &dict), "car race");
    }
}
