// outputs noise type (alpha) for given 

fn compute_delta(zs: &[f64]) -> f64{
    // compute mean
    let zbar = zs.iter().sum::<f64>() / (zs.len() as f64);

    let nom = (1..zs.len()).collect::<Vec<_>>().iter()
        .map(|i| (zs[*i-1]-zbar)*(zs[*i]-zbar)).sum::<f64>();

    let denom = zs.iter().map(|z| z-zbar).map(|d| d*d).sum::<f64>();

    let r1 = nom/denom;

    r1 / (1.+r1)
}

fn compute_z0(xs: &[f64], m: usize) -> Vec<f64>{
    let mut zs = Vec::new();
    let stride = xs.len()/m;
    
    for i in 0..stride{
        zs.push(xs[i*m]);
    }

    zs
}

// returns alpha, the power noise exponent in Sy(f) space
// assume phase data as input
pub fn lag1_single(xs: &[f64], m: usize, dmin: usize, dmax: usize) -> f64{
    
    let mut zs = compute_z0(&xs, m);

    let mut d: usize = 0;

    while true{
        let delta = compute_delta(&zs);
        if d >= dmin && (delta < 0.25 || d >= dmax){
            return -2.*(delta + (d as f64)) + 2.;
        }
        else{
            // update zs accrording to algorithm
            zs = (1..zs.len()).collect::<Vec<_>>().iter().map(|i| zs[*i]-zs[*i-1]).collect::<Vec<_>>();
            d = d+1;
        }
    }

    panic!("one should never be here...");
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::test_suite::generate_phase;

    #[test]
    fn test_suite() {
        let phases = generate_phase(); // test_suite is white frequency noise
        
        let p = lag1_single(&phases, 1, 0, 2, );

        assert_eq!(p.floor(), 0.0);
    }
}