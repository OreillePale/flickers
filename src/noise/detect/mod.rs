pub mod lag1;
pub mod b1;

use crate::enums::{*};

fn lag1b1(xs: &[f64], m: usize, tau0: f64, dmin: usize, dmax: usize) -> f64{
    let samples = xs.len()/m-1;

    if samples >= 30{
        return lag1::lag1_single(&xs, m, dmin, dmax);
    }

    b1::b1_noise_id_single(&xs, m ,tau0)
}

pub fn noise_id(xs: &[f64], m: usize, tau0: f64, strategy: NoiseId) -> f64{
    match strategy{
        NoiseId::Lag1(dmin,dmax) => lag1::lag1_single(&xs, m, dmin, dmax),
        NoiseId::B1 => b1::b1_noise_id_single(&xs, m ,tau0),
        NoiseId::Lag1B1(dmin,dmax) => lag1b1(&xs, m, tau0, dmin, dmax),
        NoiseId::Default => panic!("Default noise id algorithm shot not be here"),
        _ => panic!("Noise Id algorithm {:?} is not (yet) implemented", strategy)
    } 
}