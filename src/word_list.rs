use crate::alphagram::Alphagram;

use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

pub fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

pub fn words_with_alphagrams(word_list: &Vec<String>) -> Vec<(&String, Alphagram)> {
    let list: Vec<_> = word_list.iter().map(|word| (word, Alphagram::new(word))).collect();
    list
}

pub fn found_within(word_list: Vec<(&String, Alphagram)>, phrase: String) -> Vec<(&String, Alphagram)> {
    let phrase_alphagram = Alphagram::new(&phrase);
    let word_list: Vec<_> = word_list.iter().filter(|(_word, word_alphagram)| phrase_alphagram.contains(word_alphagram)).cloned().collect();
    word_list
}

