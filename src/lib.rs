use std::fs::File;
use std::io::{Read, Result};

use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyTuple};
use tar::{Archive, Entries, Entry};


pub struct Sample{
    path: String,
    data: Vec<u8>
}


impl Sample {
    fn new(path: String, data: Vec<u8>) -> Self {
        Sample { path, data }
    }
}


impl IntoPy<PyObject> for Sample {
    fn into_py(self, py: Python<'_>) -> PyObject {
        let py_path = self.path.into_py(py);
        let py_data = PyBytes::new(py, &self.data).into_py(py);
        PyTuple::new(py, [py_path, py_data]).into()
    }
}


trait ReadEntry<R: Read> {
    fn read(self) -> Result<Sample>;
}


impl<R: Read> ReadEntry<R> for Entry<'_, R> {
    fn read(mut self) -> Result<Sample> {
        let mut data = Vec::new();
        let path = self.path()?.to_str().unwrap().to_string();
        self.read_to_end(&mut data)?;
        Ok(Sample::new(path, data))
    }
}


pub struct TarReader<'a, File: std::io::Read> {
    pub entries: Entries<'a, File>,
    archive_ptr: *mut Archive<File>
}


impl TarReader<'_, File> {
    pub fn new(path: &str) -> Self {
        let file = File::open(path).unwrap();
        let archive = Box::new(Archive::new(file));
        let archive_ptr = Box::into_raw(archive);
        let ptr_copy = archive_ptr.clone();
        unsafe {
            let entries = archive_ptr.as_mut().unwrap().entries().unwrap();
            TarReader { entries, archive_ptr: ptr_copy}
        }
    }

    fn close(&self) {
        let archive_ptr = self.archive_ptr;
        unsafe {
            let _ = Box::from_raw(archive_ptr);
        }
    }
}


impl Iterator for TarReader<'_, File> {
    type Item = Result<Sample>;

    fn next(&mut self) -> Option<Self::Item> {
        self.entries.next().map_or_else(
            || {
            self.close();
            None
        },
        |entry| {
            match entry {
                Ok(entry) => Some(entry.read()),
                Err(e) => Some(Err(e))
            }
        }
        )
    }
}


#[pyclass(unsendable)]
struct WrappedTarReader {
    iter: TarReader<'static, File>,
}


#[pymethods]
impl WrappedTarReader {
    #[new]
    fn py_new(path: &str) -> Self {
        WrappedTarReader { iter: TarReader::new(path) }
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
fn PyTarRs(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<WrappedTarReader>()?;
    Ok(())
}