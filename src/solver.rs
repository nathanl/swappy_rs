use crate::alphagram::Alphagram;
use crate::candidate_anagram::CandidateAnagram;
use crate::priority::Priority;
use crate::node::Node;
use crate::word_list;
use std::collections::HashMap;
// use std::time::Instant;

/* We are searching the tree of possible anagrams which use our word list and phrase.
 * Our tree could look like this, where a node is "found words / remaining letters". A leaf node is
 * one where either we've used all the letters (successful leaf - anagram) or there is no word we
 * can spell with the remaining letters (failed leaf)
 *
 * - racecar /
 *   - ace / rcar
 *       - ace car / r [failed leaf]
 *   - race / car
 *       - race a / cr  [failed leaf]
 *       - race car /  [successful leaf]
 *
 * Our data structures are:
 *   - Alphagram
 *   - CandidateAnagram has the remaining characters and the words found so far (as a list of
 *   numbers)
 */


pub fn new_anagrams_for(
    user_input: String,
    word_list: &Vec<String>,
    requested_length: usize,
) -> Vec<String> {
    let mut anagrams: Vec<String> = vec![];
    eprintln!("Prepping the word list");
    let word_list = word_list::words_with_alphagrams(word_list);
    let word_list = word_list::found_within(word_list, user_input.clone());
    eprintln!("Prepped the word list");

    // New plan:
    if user_input == "".to_string() { return vec![] };
    let mut good_leaf_nodes: Vec<Node> = vec![];
    let mut current_node: Node = Node::new(vec![0]);
    let mut node_map: HashMap<Node, Alphagram> = HashMap::new();
    node_map.insert(current_node, Alphagram::new(&user_input));
    let last_word_index = word_list.len();

    loop {
        if result_accumulator.len() >= requested_length {

            for node in good_leaf_nodes {
                anagrams.push(node_to_string(&priority, &word_list));
            }

            return anagrams;
        }
        // return if results full
        // return if current_node = root ([])
        // - get alphagram from parent_node(current_node)
        // - try to remove the last word in current_node
        //   - if can
        //     - if letters remaining is empty
        //       - add current_node to results
        //       - set current_node = next_node(current_node, last_word_index)
        //       - continue
        //     - else
        //       - add current_node to node_map
        //       - set current_node = child_node(current_node)
        //       - continue
        //   - if can't
        //     - set current_node = next_node(current_node, last_word_index)
        //     - continue
    }
    return vec![];
}


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
        result_accumulator.push(c.words_found);
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

fn node_to_string(node: &Node, word_list: &Vec<(&String, Alphagram)>) -> String {
    let mut result = String::new();
    for number in node.clone().into_iter() {
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
