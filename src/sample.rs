use pyo3::{IntoPy, PyObject, Python, ToPyObject};
use pyo3::types::{PyBytes, PyDict, PyTuple};


pub struct SampleFile {
    pub path: String,
    pub data: Vec<u8>
}


impl SampleFile {
    pub(crate) fn new(path: String, data: Vec<u8>) -> Self {
        SampleFile { path, data }
    }

    pub(crate) fn path_info(&self) -> (String, String) {
        let mut path = self.path.splitn(2, '.');
        let key = path.next().unwrap().to_string();
        let file = path.next().unwrap().to_string();
        (key, file)
    }
}


impl IntoPy<PyObject> for SampleFile {
    fn into_py(self, py: Python<'_>) -> PyObject {
        let py_path = self.path.into_py(py);
        let py_data = self.data.into_py(py).to_object(py);
        PyTuple::new(py, [py_path, py_data]).into()
    }
}


pub struct Sample {
    pub key: String,
    pub frames: Option<Vec<Vec<u8>>>,
    pub target: Option<Vec<u8>>,
}


impl Sample {
    pub(crate) fn new(key: String) -> Self {
        Sample { key, frames: None, target: None }
    }

    fn _add_sample(&mut self, data: Vec<u8>) {
        match self.frames {
            Some(ref mut frames) => frames.push(data),
            None => self.frames = Some(vec![data])
        };
    }

    fn _set_target(&mut self, target: Vec<u8>) {
        match self.target {
            Some(_) => panic!("Target already set"),
            None => self.target = Some(target)
        };
    }

    pub(crate) fn update (&mut self, file_name: &str, data: Vec<u8>) {
        if file_name.starts_with("frame") {
            self._add_sample(data);
        }
        else if file_name.starts_with("target") {
            self._set_target(data);
        }
    }
}


impl IntoPy<PyObject> for Sample {
    fn into_py(self, py: Python<'_>) -> PyObject {
        let sample= PyDict::new(py);

        let key = self.key.to_object(py);
        let target = PyBytes::new(py, &*self.target.unwrap()).to_object(py);
        let frames: Vec<&PyBytes> = self.frames.unwrap().into_iter().map(|b|
            { PyBytes::new(py, &*b) }).collect();

        sample.set_item("__key__", key).unwrap();
        sample.set_item("__target__", target).unwrap();
        sample.set_item("frames", frames.to_object(py)).unwrap();
        sample.into()
    }
}