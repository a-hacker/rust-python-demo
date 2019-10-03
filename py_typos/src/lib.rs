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
}


#[pymodule]
fn py_typos(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<WordProfiler>()?;
    Ok(())
}