/// Inverse chi-square CDF (quantile function) using binary search.
/// p is the upper-tail probability (e.g., 0.05 for 95% confidence).
/// this is copied from chatGPT, don't kill me please
pub fn chi_square_inv(p: f64, df: f64) -> f64 {
    assert!(p > 0.0 && p < 1.0, "p must be between 0 and 1");
    assert!(df > 0.0, "Degrees of freedom must be > 0");

    // Binary search range
    let mut low = 0.0;
    let mut high = 1000.0; // should be large enough for most df
    let tol = 1e-10;

    while high - low > tol {
        let mid = (low + high) / 2.0;
        let p_mid = chi_square_sf(mid, df);
        if p_mid > p {
            low = mid;
        } else {
            high = mid;
        }
    }
    (low + high) / 2.0
}

/// Survival function (1 - CDF) for chi-square distribution
fn chi_square_sf(x: f64, k: f64) -> f64 {
    1.0 - lower_incomplete_gamma(k / 2.0, x / 2.0) / gamma(k / 2.0)
}

// Gamma and incomplete gamma as before:
fn gamma(z: f64) -> f64 {
    let p: [f64; 6] = [
        676.5203681218851,
        -1259.1392167224028,
        771.32342877765313,
        -176.61502916214059,
        12.507343278686905,
        -0.13857109526572012,
    ];
    let g = 7.0;
    if z < 0.5 {
        std::f64::consts::PI / ((std::f64::consts::PI * z).sin() * gamma(1.0 - z))
    } else {
        let mut x = 0.99999999999980993;
        let mut z = z - 1.0;
        for (i, &pval) in p.iter().enumerate() {
            x += pval / (z + (i as f64) + 1.0);
        }
        let t = z + g + 0.5;
        (2.0 * std::f64::consts::PI).sqrt() * t.powf(z + 0.5) * (-t).exp() * x
    }
}

fn lower_incomplete_gamma(s: f64, x: f64) -> f64 {
    let mut sum = 1.0 / s;
    let mut term = sum;
    let mut n = 1.0;
    while term.abs() > 1e-15 {
        term *= x / (s + n);
        sum += term;
        n += 1.0;
    }
    sum * x.powf(s) * (-x).exp()
}
