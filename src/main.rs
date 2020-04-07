mod alphagram;
mod priority;
mod solver;
mod candidate_anagram;
mod word_list;

fn main() {
    // let word_list = word_list::lines_from_file("/usr/share/dict/words");
    let word_list = word_list::lines_from_file("words.txt");
    let results = solver::anagrams_for(&"meathead", &word_list, 4);
    for r in results {
        println!("result is {}", r);
    }
}
