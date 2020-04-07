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

