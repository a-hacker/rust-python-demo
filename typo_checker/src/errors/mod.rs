extern crate fst;
extern crate fst_levenshtein;

use fst_levenshtein::Error as LevError;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct DictionaryError {
    message: String
}

impl DictionaryError {
    pub fn new(msg: &str) -> DictionaryError {
        DictionaryError{ message: msg.to_string() }
    }
}

impl fmt::Display for DictionaryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for DictionaryError {
    fn description(&self) -> &str {
        &self.message
    }
}

impl From<fst::Error> for DictionaryError {
    fn from(err: fst::Error) -> Self {
        DictionaryError::new(err.description())
    }
}

impl From<LevError> for DictionaryError {
    fn from(err: LevError) -> Self {
        DictionaryError::new(err.description())
    }
}