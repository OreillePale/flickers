use crate::enums::*;

#[cfg_attr(feature = "serialization", derive(serde::Serialize))]
#[derive(Debug,Clone)]
pub struct DevResult{
    pub dev: Option<DevType>,
    pub taus: Option<Vec<f64>>,
    pub devs: Option<Vec<f64>>,
    pub ns: Option<Vec<usize>>,
    pub noise_id: Option<NoiseId>,
    pub alphas: Option<Vec<f64>>,
    pub edfs: Option<Vec<f64>>,
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
}

