use crate::dev::DevEngine;
use crate::dev::adev::AdevEngine;

fn alpha2mu(alpha: i32) -> i32{
    match alpha {
        2 => -2,
        1 => -2,
        0 => -1,
        -1 => 0,
        -2 => 1,
        -3 => 2,
        _ => panic!("alpha = {} is not supported", alpha)
    }
}

// we do not care about r != 1., yet
fn b1_th(N: usize, alpha: i32) -> f64{

    if N == 2{
        return 1.;
    }

    let mu = alpha2mu(alpha);
    let N64 = N as f64;

    if mu == 0{
        return 0.5*N64*(N64-2.).ln()/(N64-1.);
    }
    
    // use analytical formula
    0.5*N64*(1.-N64.powi(mu)) / (1.-2.0_f64.powi(mu))/(N64-1.)
}

fn b1_exp(xs: &[f64], m: usize, tau0: f64) -> f64{
    // calculate adev
    let engine = AdevEngine::new();
    let avar = engine.compute_one(&xs, m, tau0).powi(2);

    // calculate standard variance; maybe not so easy when using phase data
    // not so easy for phase data
    let strides = xs.len()/m-1;
    
    let mu = (xs[m*strides]-xs[0])/tau0/(strides as f64); // it is easy to compute the mean from the phases
    let m64 = m as f64;
    let var = (0..strides).collect::<Vec<_>>().iter().map(|i| (xs[i*m+m]-xs[i*m]-mu)/tau0/m64).map(|d| d*d).sum::<f64>() / (strides as f64);

    var / avar
}


fn argmin<T: std::cmp::PartialOrd>(xs: &[T]) -> usize{
    let mut k = 0;

    for i in 1..xs.len(){
        if xs[i] < xs[k]{
            k = i;
        }
    }
    
    k
}

// returns alpha
pub fn b1_noise_id_single(xs: &[f64], m: usize, tau0: f64) -> f64{
    let b1_meas = b1_exp(&xs, m, tau0);
    let alphas: Vec<i32> = vec![-3,-2,-1,0,1,2];
    let scores = alphas.iter().map(|al| b1_th(xs.len(),*al)-b1_meas).map(|d| d*d).collect::<Vec<_>>();
    
    let i = argmin::<_>(&scores);

    alphas[i] as f64
}

fn mean_from_phase(xs: &[f64], m: usize, tau0: f64) -> f64{
    let strides = xs.len()/m-1;
    
    let mu = (xs[m*strides]-xs[0])/tau0/(strides as f64); // it is easy to compute the mean from the phases

    mu
}

fn phase2freq(xs: &[f64]) -> Vec<f64>{
    let mut ys = Vec::new();

    for i in 1..xs.len(){
        ys.push(xs[i]-xs[i-1]);
    }

    ys
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::test_suite::generate_phase;

    #[test]
    fn test_suite() {
        let phases = generate_phase(); // test_suite is white frequency noise
        
        let b1 = b1_exp(&phases, 10, 1.);

        assert_eq!(b1, 0.8696789581697049);
    }

    #[test]
    fn mean(){
        let phases = generate_phase();
        let ys = phase2freq(&phases);

        let mu1 = ys.iter().sum::<f64>() / (ys.len() as f64);
        let mu2 = mean_from_phase(&phases, 1, 1.);

        assert_eq!(mu1, mu2);
    }

    #[test]
    fn test_suite_noise_id() {
        let phases = generate_phase(); // test_suite is white frequency noise
        
        let al = b1_noise_id_single(&phases,1,1.);
        // test suite should be wfm for m=1
        assert_eq!(al.floor(), 0.0);
    }
}