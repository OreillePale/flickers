use pyo3::prelude::*;
use pyo3::{Python,Py,PyAny};
use pyo3::types::PyDict;

// use crate::py_module::translate::*;
use crate::dev_computer::*; // we are not sharing this
use crate::utils::*;
use serde_pyobject::{to_pyobject,from_pyobject};
use crate::enums::*;

fn parse_enum<T>(val: Option<Bound<'_, PyAny>>, default: T) -> T
where
    T: Clone + for<'de> serde::Deserialize<'de>,
{
    match val {
        None => default,
        Some(obj) => from_pyobject(obj).unwrap(),
    }
}

// now I should be able to ROCK and finally do some testing !
// I need to choose if I want cis or errs... I'd prefer errs actually, it make more sense to me
// would be stupid to accept both
#[pyfunction]
#[pyo3(signature=(data, dev_type=None, tau0=None, data_type=None, afs=None, noise_id=None))]
pub fn compute(py: Python<'_>, 
    data: Vec<f64>, 
    dev_type: Option<Bound<'_, PyAny>>, 
    tau0: Option<f64>, 
    data_type: Option<&str>, 
    afs: Option<Bound<'_, PyAny>>, 
    noise_id: Option<Bound<'_, PyAny>>) -> PyResult<Py<PyDict>> 
{
    let tau0 = tau0.unwrap_or(1.);

    // handle datatype
    let phases = match data_type.unwrap_or("phase") {
        "phase" => data.clone(),
        "freq" => frequency2phase(&data, tau0),
        _ => panic!("Specified data_type does not exist. Please either put `phase` or `freq`")
    };

    let dev_type: DevType = parse_enum(dev_type, DevType::Adev);
    let afs: Afs = parse_enum(afs, Afs::All);
    let noise_id: NoiseId = parse_enum(noise_id, NoiseId::Default);
    
    // compute deviation of interest
    let result_rs = DevComputer::default()
        .with_phases(&phases)
        .with_tau0(tau0)
        .with_dev(dev_type)
        .with_afs(afs)
        .with_noise_id(noise_id)
        .compute();

    // return as python dictionary, maybe can be improved, but thank you Serde...
    let py_obj = to_pyobject(py, &result_rs)?;
    let dict = py_obj.downcast::<PyDict>()?;

    Ok(dict.clone().into())
}

pub fn init_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(compute, m)?)?;
    Ok(())
}