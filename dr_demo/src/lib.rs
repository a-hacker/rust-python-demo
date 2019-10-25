use pyo3::prelude::*;
use pyo3::exceptions;
use pyo3::wrap_pyfunction;
use pyo3::PyObject;

use pyo3::types::{PyList, PyTuple, PyDict, PyAny, PyFloat};

#[pyfunction]
fn hello_world() -> PyResult<String> {
    Ok(String::from("Hello, world!"))
}

#[pyfunction]
fn add_one(x: i32) -> PyResult<i32> {
    Ok(x + 1)
}

#[pyfunction]
fn call_function(py: Python, x: PyObject) -> PyResult<PyObject> {
    x.call0(py)
}

#[pyfunction(args="*")]
fn sum_floats(args: Option<&PyTuple>) -> PyResult<f64> {
    let sum = match args {
        Some(a) => a.into_iter().map(|x| x.downcast_ref::<PyFloat>())
                          .filter_map(Result::ok).map(PyFloat::value).sum(),
        None => 0.0
    };
    Ok(sum)
}

#[pyclass]
struct MyClass {
   pub num: i32,
}

#[pymethods]
impl MyClass {
     #[new]
     fn new(obj: &PyRawObject, num: i32) {
         obj.init({
             MyClass {
                 num,
             }
         });
     }
}

#[pyclass]
struct AccessibleMyClass {
   num: i32,
}

#[pymethods]
impl AccessibleMyClass {
     #[new]
     fn new(obj: &PyRawObject, num: i32) {
         obj.init({
             AccessibleMyClass {
                 num,
             }
         });
     }

    #[getter]
    fn num(&self) -> PyResult<i32> {
        Ok(self.num)
    }
}



#[pymodule]
fn dr_demo(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(hello_world))?;
    m.add_wrapped(wrap_pyfunction!(add_one))?;
    m.add_wrapped(wrap_pyfunction!(call_function))?;
    m.add_wrapped(wrap_pyfunction!(sum_floats))?;
    m.add_class::<MyClass>()?;
    m.add_class::<AccessibleMyClass>()?;
    Ok(())
}