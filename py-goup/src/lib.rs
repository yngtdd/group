use pyo3::prelude::*;
use group::{sum, adder as adder_rs};

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

/// Sum all numbers from 0..range
///
/// Args:
///     range: the upper bound of the numbers we sum
#[pyfunction]
fn adder(range: usize) -> PyResult<usize> {
    let result = adder_rs(range);
    Ok(result)
}

/// A Python module implemented in Rust.
#[pymodule]
fn group(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(adder, m)?)?;
    Ok(())
}
