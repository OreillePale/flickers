use pyo3::prelude::*;

use crate::test_suite::*;

#[pyfunction]
pub fn phases() -> PyResult<Vec<f64>> {
    Ok(generate_phase())
}

#[pyfunction]
pub fn freqs() -> PyResult<Vec<f64>> {
    Ok(generate_frequency())
}

pub fn init_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(phases, m)?)?;
    m.add_function(wrap_pyfunction!(freqs, m)?)?;
    Ok(())
}