mod alphagram;
mod candidate_anagram;
mod solver;
mod found_words;
mod word_list;
use std::env;
use std::path::Path;
use std::process;
use shellexpand;
#[macro_use]
extern crate lazy_static;

fn main() {
    let args: Vec<String> = env::args().collect();
    let phrase: &str;
    let removal: &str;
    match args.get(1) {
        Some(input) => {
            phrase = &input;
        }
        None => {
            print_usage();
            std::process::exit(1);
        }
    }

    // TODO remove from input phrase so we can pull out words we like
    match args.get(2) {
        Some(input) => {
            removal = &input;
            println!("removing '{}' from the input phrase", removal);
        }
        None => {
            // do nothing
        }
    }

    let word_list_file = match env::var("WORDLIST") {
        Ok(filename) => filename.to_string(),
        Err(_) => {
            let f = "~/.swappy_wordlist";
            println!("using wordlist file {}", f);
            shellexpand::tilde(f).to_string()
        }
    };

    if !Path::new(&word_list_file).exists() {
        println!("word list file '{}' does not exist", word_list_file);
        process::exit(1);
    }

    let limit: usize = match env::var("LIMIT") {
        Ok(val) => val.parse::<usize>().unwrap(),
        Err(_) => 100,
    };

    let word_list = word_list::lines_from_file(word_list_file);
    let (partial_anagram_count, results) = solver::anagrams_for(phrase.to_string(), &word_list, limit);
    for r in &results {
        println!("{}", r);
    }
    eprintln!("{} partial anagrams considered, {} complete anagrams found (limit {})", partial_anagram_count, results.len(), limit);
}

fn print_usage() {
    println!(
        "
    USAGE:
       swappy 'my phrase'
       LIMIT=3 WORDLIST=/some/file.txt swappy 'my phrase'
    "
    );
}
