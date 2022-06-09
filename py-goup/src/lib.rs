use pyo3::prelude::*;
use group::sum;

/// Formats the sum of two numbers as string.
///
/// Args:
///     a: an integer
///     b: another integer
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    let result = sum(a, b);
    Ok((result).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn group(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}
