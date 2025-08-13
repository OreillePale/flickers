pub fn generate_frequency() -> Vec<f64>{
    let mut ret: [usize; 1000] = [0;1000];

    ret[0] = 1234567890;

    for i in 1..1000{
        ret[i] = 16807*(ret[i-1]) % 2147483647;
    }

    ret.iter().map(|x| (*x as f64) / 2147483647.0).collect::<Vec<_>>()
}

pub fn generate_phase() -> Vec<f64>{
    let ys = generate_frequency();

    let mut ret = vec![0.;ys.len()+1];
    ret[0] = 0.;

    for i in 0..ys.len(){
        ret[i+1] = ret[i] + ys[i];
    }

    ret
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::test_suite::generate_frequency;

    fn min(xs: &[f64]) -> f64{
        let mut mmin = xs[0];

        for i in 1..xs.len(){
            if xs[i] < mmin{
                mmin = xs[i];
            }
        }

        mmin
    }

    

    fn max(xs: &[f64]) -> f64{
        let mut mmin = xs[0];

        for i in 1..xs.len(){
            if xs[i] > mmin{
                mmin = xs[i];
            }
        }

        mmin
    }

    fn mean(xs: &[f64]) -> f64{
        xs.iter().sum::<f64>() / (xs.len() as f64)
    }

    #[test]
    fn test_testsuite() {
        let phases = generate_frequency();

        // corrected for rounding errors
        assert_eq!(phases[0] as f32, 0.5748904732);
        assert_eq!(min(&phases) as f32, 0.0013717599);
        assert_eq!(max(&phases) as f32, 9.957453e-01);
        assert_eq!(mean(&phases) as f32, 0.48977447);
    }
}