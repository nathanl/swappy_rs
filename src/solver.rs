use crate::alphagram::Alphagram;
use crate::candidate_anagram::CandidateAnagram;
use crate::priority::Priority;
use crate::word_list;
use std::time::Instant;
use priority_queue::PriorityQueue;
// use keyed_priority_queue::KeyedPriorityQueue as PriorityQueue;

pub fn anagrams_for(
    user_input: String,
    word_list: &Vec<String>,
    requested_length: usize,
) -> Vec<String> {
    let mut attempts = 0;
    let mut results = vec![];
    eprintln!("Prepping the word list");
    let word_list = word_list::words_with_alphagrams(word_list);
    let word_list = word_list::found_within(word_list, user_input.clone());
    eprintln!("Prepped the word list");
    let mut pq = PriorityQueue::new();

    let mut now = Instant::now();

    let c = CandidateAnagram::new(&user_input);
    pq.push(c, Priority::new(vec![]));

    loop {
        attempts += 1;
        if attempts % 100_000 == 0 {
            let elapsed = now.elapsed().as_millis();
            eprintln!(
                "{} attempts, {} results, pq length {}, elapsed {:?}",
                attempts,
                results.len(),
                pq.len(),
                elapsed
            );
            now = Instant::now();
        }
        let popped = pq.pop();
        if popped.is_none() {
            return results;
        }
        let (candidate, priority) = popped.unwrap();
        if candidate.is_complete() {
            let result_string = priority_to_string(&priority, &word_list);
            // eprintln!("{}", result_string);
            results.push(result_string);
            if results.len() >= requested_length {
                return results;
            }
            continue;
        }
        let word_index = candidate.next_word;
        let word_list_alphagram = &word_list[word_index].1;
        let without_result = candidate.without(&word_list_alphagram, word_index);
        match without_result {
            Ok(new_candidate) => {
                pq.push(new_candidate, priority.plus(word_index));
                ()
            }
            Err(_) => (),
        }

        if candidate.next_word + 1 < word_list.len() {
            let next_candidate = candidate.advanced_by(1);
            pq.push(next_candidate, priority);
            // println!("pq len {}", pq.len());
        }
    }
}

fn priority_to_string(priority: &Priority, word_list: &Vec<(&String, Alphagram)>) -> String {
    let mut result = String::new();
    for number in priority.clone().into_iter() {
        result.push_str(&word_list[number].0.clone());
        result.push_str(" ");
    }
    return result;
    //priority.clone().into_iter().map(|p| word_list[p]).clone().collect::<Vec<String>>().join(&" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::priority::Priority;
    use crate::word_list;

    fn word_list(vec: Vec<&str>) -> Vec<String> {
        let strings: Vec<_> = vec.iter().map(|str| str.to_string()).collect();
        strings
    }

    #[test]
    fn test_priority_to_string() {
        let word_list = word_list(vec![
            "fanhead", "car", "potatoes", "race", "floppy", "acre", "aa", "rcecr",
        ]);
        let words_with_alphagrams = word_list::words_with_alphagrams(&word_list);
        let p = Priority::new(vec![1, 3]);
        assert_eq!(priority_to_string(&p, &words_with_alphagrams), "car race ");
    }

    #[test]
    fn test_anagrams_for() {
        let word_list = word_list(vec![
            "fanhead", "car", "potatoes", "race", "floppy", "acre", "aa", "rcecr",
        ]);
        // let word_list = word_list::words_with_alphagrams(&vec!["fanhead", "car", "potatoes", "race", "floppy", "acre", "aa", "rcecr"]);
        assert_eq!(
            anagrams_for("racecar".to_string(), &word_list, 2),
            vec!["car race ", "car acre "]
        );
    }

    #[test]
    #[cfg_attr(not(feature = "benchmark"), ignore)]
    fn benchmark() {
        use std::time::Instant;
        let now = Instant::now();

        let word_list = word_list::lines_from_file("test_support/smallish_list.txt");
        let results = anagrams_for("rust language".to_string(), &word_list, 1_000);
        println!("result count {}", results.len());

        let elapsed = now.elapsed().as_millis();
        println!("Elapsed: {:?}", elapsed);
        assert!(elapsed < 7_500);
    }
}
