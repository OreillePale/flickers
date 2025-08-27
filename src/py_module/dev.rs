use pyo3::prelude::*;
use pyo3::{Python,Py};
use pyo3::types::PyDict;

use crate::enums::*;
use crate::dev_result::*;
use crate::dev_computer::*;
use serde_pyobject::{to_pyobject, pydict};

#[pyfunction]
#[pyo3(signature = (phases, tau0=1., dev_type=DevType::Adev, afs=Afs::All(), noise_id=NoiseId::Default()))]
pub fn compute(phases: Vec<f64>, tau0: f64, dev_type: DevType, afs: Afs, noise_id: NoiseId) -> PyResult<DevResult> {
    let result = DevComputer::default()
        .with_phases(&phases)
        .with_tau0(tau0)
        .with_dev(dev_type)
        .with_afs(afs)
        .with_noise_id(noise_id)
        .compute();

    Ok(result)
}

pub fn init_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(compute, m)?)?;
    Ok(())
}