use pyo3::prelude::*;
use pyo3::exceptions;

use typo_checker::Dictionary;

#[pyclass]
struct WordProfiler {
    profiler: Dictionary
}

#[pymethods]
impl WordProfiler {
    #[new]
     fn new(obj: &PyRawObject, path: &str) {
         obj.init({
             WordProfiler {
                 profiler: Dictionary::new(path),
             }
         });
     }

     fn search(&self, word: &str) -> PyResult<Vec<String>> {
         match self.profiler.search(word) {
             Ok(r) => Ok(r),
             Err(_e) => Err(exceptions::ValueError::py_err("Failed to search for input string!"))
         }
     }

     fn get_most_frequent(&self, n: usize) -> PyResult<Vec<&String>> {
         match self.profiler.get_most_frequent(n) {
             Ok(r) => Ok(r),
             Err(_e) => Err(exceptions::ValueError::py_err("Expected too many values!"))
         }
     }
}

#[pyclass]
struct ParallelWordProfiler {
    profiler: Dictionary
}

#[pymethods]
impl ParallelWordProfiler {
    #[new]
     fn new(obj: &PyRawObject, py: Python, path: &str) {
         let dict = py.allow_threads(move || Dictionary::par_new(path));
         obj.init({
             ParallelWordProfiler {
                 profiler: dict,
             }
         });
     }

     fn search(&self, word: &str) -> PyResult<Vec<String>> {
         match self.profiler.search(word) {
             Ok(r) => Ok(r),
             Err(_e) => Err(exceptions::ValueError::py_err("Failed to search for input string!"))
         }
     }

     fn get_most_frequent(&self, n: usize) -> PyResult<Vec<&String>> {
         match self.profiler.get_most_frequent(n) {
             Ok(r) => Ok(r),
             Err(_e) => Err(exceptions::ValueError::py_err("Expected too many values!"))
         }
     }
}


#[pymodule]
fn py_typos(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<WordProfiler>()?;
    m.add_class::<ParallelWordProfiler>()?;
    Ok(())
}