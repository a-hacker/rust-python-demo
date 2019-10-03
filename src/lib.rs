extern crate fst;
extern crate fst_levenshtein;
extern crate itertools;

pub mod errors;
mod utils;

use std::path::PathBuf;
use std::collections::HashMap;
use fst::{IntoStreamer, Set};
use fst_levenshtein::Levenshtein;
use errors::DictionaryError;
use utils::*;


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
}