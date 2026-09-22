use template_cke::Interpolator;
use pyo3::prelude::*;

#[pyclass(frozen)]
pub struct PyInterpolator(Interpolator);

#[pymethods]
impl PyInterpolator {
    #[new]
    pub fn new(format: &str, separator: Option<&str>) -> PyResult<Self> {
        let interpolator = Interpolator::try_new(format, separator)
            .map_err(PyErr::new::<pyo3::exceptions::PyValueError, _>)?;
        Ok(Self(interpolator))
    }

    /// Encode the given chunk index into a string.
    pub fn encode(&self, chunk_idx: Vec<u64>) -> PyResult<String> {
        self.0
            .interpolate(&chunk_idx)
            .map_err(PyErr::new::<pyo3::exceptions::PyValueError, _>)
    }
}

/// A Python module implemented in Rust.
#[pymodule]
mod _template_cke_py {
    #[pymodule_export]
    use super::PyInterpolator;
}
