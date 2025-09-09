use statrs::distribution::{ChiSquared, ContinuousCDF}; 

// maybe that is not the best way to do it
pub fn chi_square_inv(p: f64, df: f64) -> f64 {
    assert!(p > 0.0 && p < 1.0, "p must be between 0 and 1");
    assert!(df > 0.0, "Degrees of freedom must be > 0");

    // Binary search range
    let mut low = 0.0;
    let mut high = 1000.0; // should be large enough for most df
    let tol = 1e-10;

    let chi2 = ChiSquared::new(df).unwrap();

    let max_it = 10000;
    let mut i = 0;
    while high - low > tol {
        let mid = (low + high) / 2.0;
        let p_mid = chi2.sf(mid); 
        if p_mid > p {
            low = mid;
        } else {
            high = mid;
        }

        i += 1;

        if i > max_it{
            panic!("inv-chi2 binary search has reach max iter = {max_it}");
        }

        
    }
    (low + high) / 2.0
}
