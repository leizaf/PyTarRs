mod sample;
mod tar;
mod grouper;

use std::fs::File;

use pyo3::prelude::*;
use crate::grouper::GroupBy;
use crate::tar::TarReader;


#[pyclass(unsendable)]
struct PyReader {
    iter: TarReader<'static, File>,
}


#[pyclass(unsendable)]
struct GroupedPyReader {
    iter: GroupBy<TarReader<'static, File>>,
}


#[pymethods]
impl PyReader {
    #[new]
    fn py_new(path: &str) -> Self {
        let file = File::open(path).unwrap();
        PyReader { iter: TarReader::new(file) }
    }

    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<PyObject> {
        let next = slf.iter.next();
        next.map(|entry| {
            Python::with_gil(|py| {
                entry.unwrap().into_py(py)
            })
         })
    }
}


#[pymethods]
impl GroupedPyReader {
    #[new]
    fn py_new(path: &str) -> Self {
        let file = File::open(path).unwrap();
        GroupedPyReader { iter: GroupBy::new(TarReader::new(file)) }
    }

    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<PyObject> {
        let next = slf.iter.next();
        next.map(|entry| {
            Python::with_gil(|py| {
                entry.unwrap().into_py(py)
            })
         })
    }
}


#[pymodule]
fn pytarrs(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyReader>()?;
    m.add_class::<GroupedPyReader>()?;
    Ok(())
}
