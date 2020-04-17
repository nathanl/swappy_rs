mod alphagram;
mod candidate_anagram;
mod priority;
mod solver;
mod word_list;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let phrase: &str;
    match args.get(1) {
        Some(input) => {
            phrase = &input;
        }
        None => {
            print_usage();
            std::process::exit(1);
        }
    }

    let word_list_file = match env::var("WORDS") {
        Ok(filename) => filename.to_string(),
        Err(_) => "test_support/smallish_list.txt".to_string(),
    };

    let limit: usize = match env::var("LIMIT") {
        Ok(val) => val.parse::<usize>().unwrap(),
        Err(_) => 100,
    };

    let word_list = word_list::lines_from_file(word_list_file);
    let results = solver::anagrams_for(phrase.to_string(), &word_list, limit);
    for r in &results {
        println!("{}", r);
    }
    eprintln!("{} total results", results.len());
}

fn print_usage() {
    println!(
        "
    USAGE:
       cargo run 'my phrase'
       LIMIT=3 WORDS=/some/file.txt 'my phrase'
    "
    );
}
