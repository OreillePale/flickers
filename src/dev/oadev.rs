use crate::dev::DevEngine;
use crate::noise::detect::NoiseDetectionStrategy;

struct OadevEngine{

}

impl OadevEngine{
    fn new() -> OadevEngine{
        OadevEngine{}
    }
}

impl DevEngine for OadevEngine{
    const PREFERRED_NOISE_ID_STRATEGY: NoiseDetectionStrategy = NoiseDetectionStrategy::Lag1B1(0,2);

    fn name() -> &'static str{
        "oadev"
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

        match alpha.round() {
           2. => 0.5*(n64+1.)*(n64-2.*m64)/(n64-m64),
           1. => (0.5*(n64-1.)*(1./m64-0.5*(2.*m64+1.))).sqrt(),
           0. => (1.5*(n64-1.)/m64-2.*(n64-2.)/n64)*4.*m64*m64/(4.*m64*m64+5.),
           -1. => match m {
                1 => 2.*(n64-2.)*(n64-2.)/(2.3*n64-4.9),
                _ => 1.25*n64*n64/m64/(n64+3.*m64)
            },
            -2. => (n64-2.)/m64*((n64-1.).powi(2)-3.*m64*(n64-1.)+4.*m64.powi(2))/(n64-3.).powi(2),
            _ => panic!("no oadev edf estimation can be provided for noise type alpha = {}",alpha)
        }
    }

    // TODO: last thing to complete v 0.1.0
    fn err_factor(&self, alpha: f64, edf: f64) -> f64{
        unimplemented!("");
    }
}

// could me automated, see later
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::test_suite::generate_phase;
    use crate::dev::oadev::OadevEngine;
2
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
}