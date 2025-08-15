use crate::dev::DevEngine;
use crate::noise::detect::NoiseDetectionStrategy;

pub struct AdevEngine{

}

impl AdevEngine{
    pub fn new() -> AdevEngine{
        AdevEngine{}
    }
}

impl DevEngine for AdevEngine{

    fn name(&self) -> &'static str{
        "adev"
    }

    fn preferred_noise_id_metod(&self) -> NoiseDetectionStrategy{
        NoiseDetectionStrategy::Lag1B1(0,2)
    }

    fn compute_one(&self, xs: &[f64], m: usize, tau0: f64) -> f64{
        assert!(m <= self.m_max(xs.len())); // maybe this is going to make things slow, let's see

        let stop = (xs.len()-1) / m - 1;
        
        let ssum = (0..stop).collect::<Vec<_>>().iter()
            .map(|i| xs[m*i+2*m]-2.*xs[m*i+m]+xs[m*i])
            .map(|d| d*d).sum::<f64>().sqrt();

        ssum / (m as f64) / tau0 / 2.0_f64.sqrt() / (stop as f64).sqrt()
    }

    fn m_max(&self, n: usize) -> usize{
        (n-1) / 2
    }
    
    fn ns(&self, n: usize, m: usize) -> usize{
        (n-1) / m - 1
    }

    // for adev, the edf is the number of phase points for the given averaging factor
    fn edf(&self, alpha: f64, n: usize, m: usize) -> f64{

        // maybe not the most intuitive place to put that here...
        let kn = match alpha.round(){
             2.0 => 0.99,
             1.0 => 0.99,
             0.0 => 0.87,
            -1.0 => 0.77,
            -2.0 => 0.75,
            _ => panic!("alpha = {} noise is not resolved by adev",alpha),
        };

        (self.ns(n,m) as f64) / kn/kn
    }

    
    fn ci_factor(&self, edf: f64, _p: f64) -> (f64,f64){
        let cif = 1. / edf.sqrt();
        (1.-cif,1.+cif)
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::test_suite::generate_phase;
    use crate::dev::adev::AdevEngine;
    use crate::api::{*};

    #[test]
    fn test_single() {
        let phases = generate_phase();
        
        let engine = AdevEngine::new();

        assert_eq!(engine.compute_one(&phases, 1, 1.) as f32,2.922319e-01);
        assert_eq!(engine.compute_one(&phases, 10, 1.) as f32,9.9657364e-02);
        assert_eq!(engine.compute_one(&phases, 100, 1.) as f32,3.8978044e-02);

    }

    #[test]
    fn test_many() {
        let phases = generate_phase();
        
        let engine = AdevEngine::new();

        let devs = engine.compute_many(&phases, &[1,10,100], 1.);

        assert_eq!(devs[0] as f32,2.922319e-01);
        assert_eq!(devs[1] as f32,9.9657364e-02);
        assert_eq!(devs[2] as f32,3.8978044e-02);
    }

    #[test]
    fn test_suite_error(){
        let phases = generate_phase();
        let engine = AdevEngine::new();

        let result = engine.compute(&phases,1.,&MsGenerationSeed::Explicit(vec![10]));

        assert_eq!(result.taus.unwrap()[0], 10.);
        assert_eq!(result.ns.unwrap()[0],99);

        let dev = result.devs.unwrap()[0];
        assert_eq!(dev as f32, 9.9657364e-02);

        let cis = result.cis.unwrap()[0];
        let ci_low = cis.0;
        let ci_high = cis.1;

        assert_eq!((dev - ci_low) as f32, 8.713869e-03);
        assert_eq!((ci_high - dev) as f32, 8.713869e-03);
    }
}