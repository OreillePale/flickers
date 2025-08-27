#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg(feature = "python")]
use pyo3::types::PyDict;

use derive_builder::Builder;
use crate::enums::{*};

#[cfg_attr(feature = "serialization", derive(serde::Serialize))]
#[cfg_attr(feature = "python", pyclass)]
#[derive(Debug,Clone)]
pub struct DevResult{
    
    #[cfg(feature = "python")]
    #[pyo3(get, set)]
    pub dev: Option<DevType>,
    #[cfg(not(feature = "python"))]
    pub dev: Option<DevType>,

    #[cfg(feature = "python")]
    #[pyo3(get, set)]
    pub taus: Option<Vec<f64>>,
    #[cfg(not(feature = "python"))]
    pub taus: Option<Vec<f64>>,
    
    #[cfg(feature = "python")]
    #[pyo3(get, set)]
    pub devs: Option<Vec<f64>>, 
    #[cfg(not(feature = "python"))]
    pub devs: Option<Vec<f64>>, 
    
    #[cfg(feature = "python")]
    #[pyo3(get, set)]
    pub ns: Option<Vec<usize>>,
    #[cfg(not(feature = "python"))]
    pub ns: Option<Vec<usize>>,
    
    // TODO: refocator enums
    pub noise_id: Option<NoiseId>,
    
    #[cfg(feature = "python")]
    #[pyo3(get, set)]
    pub alphas: Option<Vec<f64>>,
    #[cfg(not(feature = "python"))]
    pub alphas: Option<Vec<f64>>,
    
    
    #[cfg(feature = "python")]
    #[pyo3(get, set)]
    pub edfs: Option<Vec<f64>>,
    #[cfg(not(feature = "python"))]
    pub edfs: Option<Vec<f64>>,
    
    #[cfg(feature = "python")]
    #[pyo3(get, set)]
    pub cis: Option<Vec<(f64,f64)>>,
    #[cfg(not(feature = "python"))]
    pub cis: Option<Vec<(f64,f64)>>,
}

macro_rules! getter_setter {
    ($field:ident, $with:ident, $t:ty) => {
        pub fn $field(&self) -> Option<$t> {
            self.$field.clone()
        }

        pub fn $with(mut self, val: $t) -> Self {
            self.$field = Some(val);
            self
        }
    };
}

impl DevResult{
    pub fn default() -> DevResult{
        DevResult{
            dev: None,
            taus: None,
            devs: None,
            ns: None,
            noise_id: None,
            alphas: None,
            edfs: None,
            cis: None
        }
    }

    getter_setter!(dev, with_dev, DevType);
    getter_setter!(taus, with_taus, Vec<f64>);
    getter_setter!(devs, with_devs, Vec<f64>);
    getter_setter!(ns, with_ns, Vec<usize>);
    getter_setter!(noise_id, with_noise_id, NoiseId);
    getter_setter!(alphas, with_alphas, Vec<f64>);
    getter_setter!(edfs, with_edfs, Vec<f64>);
    getter_setter!(cis, with_cis, Vec<(f64,f64)>);

    // pub fn at(&self) -> (Vec<f64>,Vec<f64>,Vec<usize>,Vec<(f64,f64)>){
    //     (self.taus.clone().unwrap(), self.devs.clone().unwrap(), self.ns.clone().unwrap(), self.cis.clone().unwrap())
    // }

}

// pymtehods must be in dedicated impl scope
#[cfg_attr(feature = "python", pymethods)]
impl DevResult{
    // TODO: error handling
    pub fn errs(&self) -> Vec<(f64,f64)>{
        let devs = self.devs.as_ref().unwrap();
        let cis = self.cis.as_ref().unwrap();
        let mut ret = Vec::with_capacity(cis.len());

        for i in 0..cis.len(){
            ret.push((devs[i]-cis[i].0, cis[i].1-devs[i]));
        }

        ret
    }
}

