extern crate fst;
extern crate fst_levenshtein;
extern crate itertools;

use std::io::prelude::*;
use std::io;
use std::fs::{self, File};
use std::path::PathBuf;
use std::env;
use std::collections::HashMap;
use fst::{IntoStreamer, Set};
use fst_levenshtein::Levenshtein;
use itertools::sorted;

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

fn create_fst_set(occurences: &HashMap<String, i32>) -> Set {
    Set::from_iter(sorted(occurences.keys())).expect("Couldn't create fst set")
}

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let test_string: String = args.pop().unwrap();
    let root_dir: PathBuf = match args.pop() {
        Some(dir) => PathBuf::from(dir),
        None => env::current_dir().unwrap(),
    };

    let all_files = scan_dir(root_dir);
    let occurences: HashMap<String, i32> = all_files.into_iter()
            .map(count_words)
            .filter_map(Result::ok)
            .fold(HashMap::new(), |mut occs, mut new_occs| {
                new_occs.drain().for_each(|(key, val)| {
                    let cur_val = occs.entry(key.to_string()).or_insert(0);
                    *cur_val += val;
                });
                occs
            });

    let typo_tester = create_fst_set(&occurences);
    let lev = Levenshtein::new(test_string.as_str(), 1).expect("Couldn't create test item");
    let matches = typo_tester.search(lev).into_stream().into_strs().expect("Couldn't get matches");
    matches.into_iter().for_each(|s| println!("{}", s))
}
