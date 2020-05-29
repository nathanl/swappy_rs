use crate::alphagram::Alphagram;
use crate::candidate_anagram::CandidateAnagram;
use crate::priority::Priority;
use crate::word_list;
use priority_queue::PriorityQueue;
use std::time::Instant;
// use keyed_priority_queue::KeyedPriorityQueue as PriorityQueue;


// I failed, Nathan.  I don't know rust well enough and the lag over this connection is pretty bad.
// Maybe you and I can try writing a DFS next time.  The general idea is

/*
dfs_visit(node_in_tree, visitor_function);
where node_in_tree is a candidate (the root being our first candidate), and dfs_visit is able to determine the node's children (
either leaves, representing finished anagrams, or not-leaves, representing successfully removing
a word from the root but there's still letters left over.)
And then visitor_function(node) says "if the node is a leaf, add it to result list."
For simplicity of typing, this code wants priority to be BACK INSIDE CandidateAnagram (we took it out because
PQ couldn't handle it being referred to by the CandidateAnagram as well as by the PQ), and priority would be
renamed to something like "anagram_so_far" or "path".
Things I don't know how to do yet, if I could even get the above code typed in:
 - bail after 100 entries
 */


/* We are searching the tree of possible anagrams which use our word list and phrase.
 * Our tree could look like this, where a node is "found words / remaining letters".
 *
 * - racecar /
 *   - race / car
 *       - race a / cr  [failed leaf]
 *       - race car /  [successful leaf]
 *   - craec / ar
 *       - racec a / r  [failed leaf]
 *
 * Our data structures are:
 *   - Alphagram
 *   - CandidateAnagram has the remainining characters and the words found so far (as a list of
 *   numbers)
 *
 * Game plan:
 * - Get rid of Priority and move the "words found so far" vec into CandidateAnagram
 * - Build a depth-first search which is like:
 *   - If I have no letters left, add to results and throw if we have enough results
 *   - else, for each child, recurse into child
 */

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
        // take out apple and remember that the next word to try is number 1 (apple)
        let without_result = candidate.without(&word_list_alphagram, word_index);
        match without_result {
            Ok(new_candidate) => {
                pq.push(new_candidate, priority.plus(word_index));
                ()
            }
            Err(_) => (),
        }

        // if we haven't run out of words, also try looking for the next word
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
