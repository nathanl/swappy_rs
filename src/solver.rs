use crate::alphagram::Alphagram;
use crate::candidate_anagram::CandidateAnagram;
use crate::priority::Priority;
use crate::word_list;
use std::time::Instant;


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
 *   - CandidateAnagram has the remaining characters and the words found so far (as a list of
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
    let mut anagrams: Vec<String> = vec![];
    eprintln!("Prepping the word list");
    let word_list = word_list::words_with_alphagrams(word_list);
    let word_list = word_list::found_within(word_list, user_input.clone());
    eprintln!("Prepped the word list");

    let c = CandidateAnagram::new(&user_input);

    let mut result_accumulator: Vec<Priority> = vec![];
    dfs(c, &mut result_accumulator, &requested_length, &word_list);

    for priority in result_accumulator {
        anagrams.push(priority_to_string(&priority, &word_list));
    }
    return anagrams;
}

pub fn dfs(c: CandidateAnagram, result_accumulator: &mut Vec<Priority>, requested_length: &usize, word_list: &Vec<(&String, Alphagram)>) {
    if &result_accumulator.len() >= requested_length {
        return;
    }
    if c.is_complete() {
        // base case: node is a leaf with no remaining chars
        result_accumulator.push(c.priority);
    }
    else {
        // recursive case: dfs into any children (may have no children if we're at a dead-end leaf with
        // remaining chars)
        for child in c.children(&word_list) {
            // println!("child {:?}", child);
            dfs(child, result_accumulator, requested_length, word_list);
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
