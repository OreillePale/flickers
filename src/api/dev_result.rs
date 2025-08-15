use derive_builder::Builder;
use crate::api::{DevType};

#[derive(Builder)]
pub struct DevResult{
    #[builder(default = "None")]
    pub taus: Option<Vec<f64>>,

    #[builder(default = "None")]
    pub devs: Option<Vec<f64>>,

    #[builder(default = "None")]
    pub ns: Option<Vec<usize>>,

    #[builder(default = "None")]
    pub alphas: Option<Vec<f64>>,

    #[builder(default = "None")]
    pub edfs: Option<Vec<f64>>,

    #[builder(default = "None")]
    pub cis: Option<Vec<(f64,f64)>>
}