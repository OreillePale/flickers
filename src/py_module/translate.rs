// this is where we translate rust objects to python objects
use pyo3::prelude::*;
use pyo3::pyclass;
use crate::dev_result;

// let's try with dev_result
#[pyclass]
pub struct DevResult{
    inner: dev_result::DevResult
}

macro_rules! pygetter {
    ($field:ident, $get:ident, $t:ty) => {
        #[pymethods]
        impl DevResult{
            #[getter]
            fn $get(&self) -> Option<$t> {
                self.inner.$field.clone().into()
            }
        }
    };
}

pygetter!(taus, get_taus, Vec<f64>);
pygetter!(devs, get_devs, Vec<f64>);
// pygetter!(ns, get_ns, Vec<usize>);
// pygetter!(alphas, get_alphas, Vec<f64>);
// pygetter!(edfs, get_edfs, Vec<f64>);
// pygetter!(cis, get_cis, Vec<(f64,f64)>);
