extern crate fst;
extern crate fst_levenshtein;
extern crate itertools;
extern crate rayon;

pub mod errors;
mod utils;

use std::path::PathBuf;
use std::collections::HashMap;
use fst::{IntoStreamer, Set};
use fst_levenshtein::Levenshtein;
use errors::DictionaryError;
use utils::*;
use rayon::prelude::*;


pub struct Dictionary {
    pub fst: Set,
    pub occurences: HashMap<String, i32>
}

impl Dictionary {
    pub fn new(input_path: &str) -> Dictionary {
        let files = scan_dir(input_path);
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
        let fst = create_fst_set(&occurences).expect("Could not resolve word set");
        
        Dictionary {
            fst: fst,
            occurences: occurences
        }
    }

    pub fn par_new(input_path: &str) -> Dictionary {
        let files = scan_dir(input_path);
        let occurences: HashMap<String, i32> = files.into_par_iter()
            .map(count_words)
            .filter_map(Result::ok)
            .reduce(|| HashMap::new(), |mut occs, mut new_occs| {
                new_occs.drain().for_each(|(key, val)| {
                    let cur_val = occs.entry(key.to_string()).or_insert(0);
                    *cur_val += val;
                });
                occs
            });
        let fst = create_fst_set(&occurences).expect("Could not resolve word set");
        
        Dictionary {
            fst: fst,
            occurences: occurences
        }
    }

    pub fn search(&self, word: &str) -> Result<Vec<String>, DictionaryError> {
        let lev = Levenshtein::new(word, 1)?;
        let matches = self.fst.search(lev).into_stream().into_strs()?;
        Ok(matches)
    }

    pub fn get_most_frequent(&self, n: usize) -> Result<Vec<&String>, DictionaryError> {
        let word_count = self.occurences.len();
        if n >= word_count {
            return Err(DictionaryError::new("Too many values expected"));
        }

        let mut count_vec: Vec<(&String, &i32)> = self.occurences.iter().collect();
        count_vec.sort_by(|a, b| b.1.cmp(a.1));
        count_vec.truncate(n);
        Ok(count_vec.into_iter().map(|(word, _)| word).collect())
    }
}