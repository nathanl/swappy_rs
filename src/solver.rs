use crate::alphagram::Alphagram;
use crate::candidate_anagram::CandidateAnagram;
use crate::found_words::FoundWords;
use crate::word_list;

pub fn anagrams_for(
    user_input: String,
    word_list: &Vec<String>,
    requested_length: usize,
) -> (u32, Vec<String>) {
    let mut anagrams: Vec<String> = vec![];
    let mut node_count: u32 = 0;
    eprintln!("Prepping the word list");
    let word_list = word_list::words_with_alphagrams(word_list);
    let word_list = word_list::found_within(word_list, user_input.clone());
    eprintln!("Prepped the word list");
    eprintln!("Searching...");

    let c = CandidateAnagram::new(&user_input);

    let mut result_accumulator: Vec<FoundWords> = vec![];
    depth_first_search(c, &mut result_accumulator, &requested_length, &word_list, &mut node_count);

    for found_words in result_accumulator {
        anagrams.push(found_words_to_string(&found_words, &word_list));
    }
    return (node_count, anagrams);
}

pub fn depth_first_search(
    c: CandidateAnagram,
    result_accumulator: &mut Vec<FoundWords>,
    requested_length: &usize,
    word_list: &Vec<(&String, Alphagram)>,
    node_count: &mut u32
) {
    if &result_accumulator.len() >= requested_length {
        return;
    }
    if c.is_complete() {
        // Base case: node is a leaf with no remaining chars
        result_accumulator.push(c.words_found);
    } else {
        // Recursive case: search into any children.
        // Note: Rust does not have tail-call optimization, so recursion is generally not
        // encouraged. But in this case, our stack depth will be equal to however many words we
        // found in a phrase. For any reasonable input phrase and dictionary, that should be a
        // small number - like, 20 would be a lot.
        for child in c.children(&word_list) {
            *node_count = *node_count + 1;
            if *node_count % 1_000_000 == 0 {
                eprintln!("{} word checks", node_count);
            }
            depth_first_search(child, result_accumulator, requested_length, word_list, node_count);
        }
    }
}

fn found_words_to_string(found_words: &FoundWords, word_list: &Vec<(&String, Alphagram)>) -> String {
    let mut result = String::new();
    for number in found_words.clone().into_iter() {
        result.push_str(&word_list[number].0.clone());
        result.push_str(" ");
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::found_words::FoundWords;
    use crate::word_list;

    fn word_list(vec: Vec<&str>) -> Vec<String> {
        let strings: Vec<_> = vec.iter().map(|str| str.to_string()).collect();
        strings
    }

    #[test]
    fn test_found_words_to_string() {
        let word_list = word_list(vec![
            "fanhead", "car", "potatoes", "race", "floppy", "acre", "aa", "rcecr",
        ]);
        let words_with_alphagrams = word_list::words_with_alphagrams(&word_list);
        let p = FoundWords::new(vec![1, 3]);
        assert_eq!(found_words_to_string(&p, &words_with_alphagrams), "car race ");
    }

    #[test]
    fn test_anagrams_for() {
        let word_list = word_list(vec![
            "fanhead", "car", "potatoes", "race", "floppy", "acre", "aa", "rcecr",
        ]);
        let (_node_count, results) = anagrams_for("racecar".to_string(), &word_list, 2);
        assert_eq!(
            results,
            vec!["car race ", "car acre "]
        );
    }

    #[test]
    #[cfg_attr(not(feature = "benchmark"), ignore)]
    fn benchmark() {
        use std::time::Instant;
        let now = Instant::now();

        let word_list = word_list::lines_from_file("test_support/wordlist.txt");
        let (_node_count, results) = anagrams_for("rust language".to_string(), &word_list, 1_000);
        println!("result count {}", results.len());

        let elapsed = now.elapsed().as_millis();
        println!("Elapsed: {:?}", elapsed);
        assert!(elapsed < 1_000);
    }
}
