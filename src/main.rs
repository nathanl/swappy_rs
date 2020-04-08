mod alphagram;
mod candidate_anagram;
mod priority;
mod solver;
mod word_list;

fn main() {
    // let word_list = word_list::lines_from_file("/usr/share/dict/words");
    let word_list = word_list::lines_from_file(
        "/Users/nathanlong/code/mine/anagram_wordlists/smallish_list.txt",
    );
    let _results = solver::anagrams_for("hoverbeast".to_string(), &word_list, 1000);
    // for r in results {
    //     println!("result is {}", r);
    // }
}
