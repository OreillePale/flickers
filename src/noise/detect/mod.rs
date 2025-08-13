pub mod lag1;
pub mod b1;

pub enum NoiseDetectionStrategy{
    Lag1(usize,usize),
    B1,
    Rn,
    Lag1B1(usize,usize)
}

fn lag1b1(xs: &[f64], m: usize, tau0: f64, dmin: usize, dmax: usize) -> f64{
    let samples = xs.len()/m-1;

    if samples >= 30{
        return lag1::lag1_single(&xs, m, dmin, dmax);
    }

    b1::b1_noise_id_single(&xs, m ,tau0)
}

pub fn noise_id(xs: &[f64], m: usize, tau0: f64, strategy: NoiseDetectionStrategy) -> f64{
    match strategy{
        NoiseDetectionStrategy::Lag1(dmin,dmax) => lag1::lag1_single(&xs, m, dmin, dmax),
        NoiseDetectionStrategy::B1 => b1::b1_noise_id_single(&xs, m ,tau0),
        NoiseDetectionStrategy::Rn => unimplemented!("R(n) noise id strategy is not implemented (yet)"),
        NoiseDetectionStrategy::Lag1B1(dmin,dmax) => lag1b1(&xs, m, tau0, dmin, dmax)
    } 
}