use pyo3::prelude::*;
use template_cke::Interpolator;

#[pyclass(frozen)]
pub struct PyInterpolator(Interpolator);

#[pymethods]
impl PyInterpolator {
    #[new]
    pub fn new(format: &str, separator: Option<&str>) -> PyResult<Self> {
        let interpolator = Interpolator::try_new(format, separator)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        Ok(Self(interpolator))
    }

    /// Encode the given chunk index into a string.
    pub fn encode(&self, chunk_idx: Vec<u64>) -> PyResult<String> {
        self.0
            .interpolate(&chunk_idx)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
    }
}

/// A Python module implemented in Rust.
#[pymodule]
mod _template_cke_py {
    #[pymodule_export]
    use super::PyInterpolator;
}
