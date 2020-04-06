mod alphagram;
mod priority;
mod solver;
mod candidate_anagram;

use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn main() {
    // let lines = lines_from_file("/usr/share/dict/words");
    let lines = lines_from_file("words.txt");
    let results = solver::anagrams_for(&"meathead", &lines, 4);
    for r in results {
        println!("result is {}", r);
    }
}
