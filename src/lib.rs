extern crate fst;
extern crate fst_levenshtein;
extern crate itertools;

use std::io::prelude::*;
use std::io;
use std::fs::{self, File};
use std::path::PathBuf;
use std::env;
use std::collections::HashMap;
use fst::{IntoStreamer, Streamer, Set};
use fst_levenshtein::Levenshtein;
use itertools::sorted;

pub struct Dictionary {
    pub fst: Set,
    pub occurences: HashMap<String, i32>
}

impl Dictionary {
    pub fn new(input_path: &str) -> Dictionary {
        let files = scan_dir(PathBuf::from(input_path));
        let occurences: HashMap<String, i32> = files.into_iter()
            .map(count_words)
            .filter_map(Result::ok)
            .fold(HashMap::new(), |mut occs, mut new_occs| {
                new_occs.drain().for_each(|(key, val)| {
                    let cur_val = occs.entry(key.to_string()).or_insert(0);
                    *cur_val += val;
                });
                occs
            });
        let fst = create_fst_set(&occurences);
        Dictionary {
            fst: fst,
            occurences: occurences
        }
    }

    pub fn search(&self, word: &str) -> Vec<String> {
        let lev = Levenshtein::new(word, 1).expect("Couldn't create test item");
        self.fst.search(lev).into_stream().into_strs().expect("Couldn't get matches")
    }
}


fn create_fst_set(occurences: &HashMap<String, i32>) -> Set {
    Set::from_iter(sorted(occurences.keys())).expect("Couldn't create fst set")
}

fn scan_dir(base_dir: PathBuf) -> Vec<PathBuf> {
    if base_dir.is_dir() {
        fs::read_dir(base_dir).unwrap().into_iter()
            .map(|x| scan_dir(x.unwrap().path()))
            .flatten()
            .collect()
    } else {
        vec![base_dir]
    }
}

fn count_words<'a>(file_path: PathBuf) -> Result<HashMap<String, i32>, io::Error> {

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