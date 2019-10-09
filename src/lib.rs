use rust_tokenizer;
use pyo3::exceptions;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyfunction]
pub fn tokenize(text: &str) -> PyResult<Vec<Vec<String>>> {
    rust_tokenizer::tokenize(text).map_err(|err| exceptions::RuntimeError::py_err(err.to_string()))
}

#[pymodule]
fn alpino_tokenizer(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(tokenize)).unwrap();
    Ok(())
}
