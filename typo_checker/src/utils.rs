extern crate fst;
extern crate fst_levenshtein;
extern crate itertools;

use std::io::prelude::*;
use std::io;
use std::fs::{self, File};
use std::path::PathBuf;
use std::collections::HashMap;
use fst::Set;
use itertools::sorted;

pub fn create_fst_set(occurences: &HashMap<String, i32>) -> Result<Set, fst::Error> {
    Set::from_iter(sorted(occurences.keys()))
}

pub fn scan_dir(base_dir: PathBuf) -> Vec<PathBuf> {
    if base_dir.is_dir() {
        fs::read_dir(base_dir).unwrap().into_iter()
            .map(|x| scan_dir(x.unwrap().path()))
            .flatten()
            .collect()
    } else {
        vec![base_dir]
    }
}

pub fn count_words<'a>(file_path: PathBuf) -> Result<HashMap<String, i32>, io::Error> {

    let mut occurences = HashMap::new();
    if !file_path.is_file() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "Provided file path is not a file or does not exist!"))
    }

    let mut file: File = File::open(file_path)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    contents.rsplit(|c: char| !c.is_digit(10) && !c.is_alphabetic()).for_each(|word| {
        if occurences.contains_key(word) {
            let word_count = occurences.get_mut(word).unwrap();
            *word_count += 1;
        } else {
            occurences.insert(word.to_string(), 1);
        }
    });
    Ok(occurences)
}