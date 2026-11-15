use pyo3::prelude::*;
use template_cke::TemplateChunkKeyEncoding;

#[pyclass(frozen)]
pub struct PyInterpolator(TemplateChunkKeyEncoding);

#[pymethods]
impl PyInterpolator {
    #[new]
    pub fn new(template: &str, separator: Option<&str>) -> PyResult<Self> {
        let interpolator = TemplateChunkKeyEncoding::try_new(template, separator)
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
