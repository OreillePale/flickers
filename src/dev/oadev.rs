use crate::dev::DevEngine;
use crate::enums::{*};
use crate::utils::chi2::chi_square_inv;
use statrs::distribution::{ChiSquared,InverseGamma, Continuous};
use statrs::distribution::ContinuousCDF;
use statrs::statistics::Distribution;
use statrs::prec;

pub struct OadevEngine{

}

impl OadevEngine{
    pub fn new() -> OadevEngine{
        OadevEngine{}
    }
}

impl DevEngine for OadevEngine{
    fn dev(&self) -> DevType{
        DevType::Oadev
    }

    fn preferred_noise_id_metod(&self) -> NoiseId{
        NoiseId::Lag1B1(0,2)
    }

    fn compute_one(&self, xs: &[f64], m: usize, tau0: f64) -> f64{
        assert!(m <= self.m_max(xs.len())); // maybe this is going to make things slow, let's see

        let stop = xs.len() - 2*m;
        
        let ssum = (0..stop).collect::<Vec<_>>().iter()
            .map(|i| xs[*i+2*m]-2.*xs[*i+m]+xs[*i])
            .map(|d| d*d).sum::<f64>().sqrt();

        ssum / (m as f64) / tau0 / 2.0_f64.sqrt() / (stop as f64).sqrt()
    }

    fn m_max(&self, n: usize) -> usize{
        (n-1) / 2
    }

    fn ns(&self, n: usize, m: usize) -> usize{
        n - 2*m
    }

    // page 49 of the handbook of frequency analysis
    fn edf(&self, alpha: f64, n: usize, m: usize) -> f64{
        let n64 = n as f64;
        let m64 = m as f64;

        let r = match alpha.round() {
           2. => 0.5*(n64+1.)*(n64-2.*m64)/(n64-m64),
           1. => ((0.5*(n64-1.)/m64).ln() * (0.25*(2.*m64+1.)*(n64-1.)).ln()).sqrt().exp(),//(0.5*(n64-1.)*(1./m64-0.5*(2.*m64+1.))).sqrt(),
           0. => ( 1.5*(n64-1.)/m64 - 2.*(n64-2.)/n64 ) * 4.*m64*m64 / (4.*m64*m64+5.),
           -1. => match m {
                1 => 2.*(n64-2.)*(n64-2.)/(2.3*n64-4.9),
                _ => 1.25*n64*n64/m64/(n64+3.*m64)
            },
            -2. => (n64-2.)/m64*((n64-1.).powi(2)-3.*m64*(n64-1.)+4.*m64.powi(2))/(n64-3.).powi(2),
            _ => panic!("no oadev edf estimation can be provided for noise type alpha = {}",alpha)
        };

        // println!("{},{},{},{}",alpha,n,m,r);

        r
    }

    fn ci_factor(&self, edf: f64, _p: f64) -> (f64,f64){
        let chi2 = ChiSquared::new(edf).unwrap();

        let lo2 = edf/chi_square_inv(_p,edf);
        let hi2 = edf/chi_square_inv(1.-_p,edf);

        (lo2.sqrt(),hi2.sqrt())
    }
}

// could me automated, see later
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::test_suite::generate_phase;
    use crate::dev::oadev::OadevEngine;
    use crate::enums::{*};
    use crate::dev_computer;

    #[test]
    fn test_single() {
        let phases = generate_phase();
        
        let engine = OadevEngine::new();

        assert_eq!(engine.compute_one(&phases, 1, 1.) as f32,2.922319e-01);
        assert_eq!(engine.compute_one(&phases, 10, 1.) as f32,9.159953e-02);
        assert_eq!(engine.compute_one(&phases, 100, 1.) as f32,3.241343e-02);

    }

    #[test]
    fn test_many() {
        let phases = generate_phase();
        
        let engine = OadevEngine::new();

        let ms: [usize; 3] = [1,10,100];

        let devs1 = engine.compute_many(&phases, &[1,10,100], 1.);
        let devs2 = ms.map(|m| engine.compute_one(&phases,m,1.));

        for i in 0..ms.len(){
            assert_eq!(devs1[i], devs2[i]);
        }
    }

    fn round_n_digits(x: f64, n: i32) -> f64{
        let mut ret = x*10.0_f64.powi(n);
        ret = ret.round_ties_even();

        ret / 10.0_f64.powi(n)
    }

    #[test]
    fn table32(){
        let phases = generate_phase();
        
        let engine = OadevEngine::new();

        let res = engine.compute(&phases, 1.0, &Afs::Explicit(vec![10]), NoiseId::Default);

        assert_eq!(res.devs.unwrap()[0] as f32, 9.159953e-02);
        assert_eq!(res.ns.unwrap()[0], 981);
        assert_eq!(res.alphas.unwrap()[0].round(), 0.0); 
        assert_eq!(round_n_digits(res.edfs.unwrap()[0],3), 146.177); 

        // check errors
        let (ci_low,ci_high) = res.cis.unwrap()[0];

        // this test would not pass because stable32 has error when estimating chi2 
        // (see A. Wallin blog: https://www.anderswallin.net/2020/12/fun-with-chi-squared/)
        // therefore values from Table 32 cannot be used as references
        assert_eq!(ci_low as f32,8.223942e-02);
        assert_eq!(ci_high as f32,1.035201e-01);
    }
}