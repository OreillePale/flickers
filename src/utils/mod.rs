pub mod chi2;
pub mod ms;

pub fn frequency2phase(ys: &[f64], tau0: f64) -> Vec<f64>{
    let mut ret = vec![0.;ys.len()+1];
    ret[0] = 0.;

    for i in 0..ys.len(){
        ret[i+1] = ret[i] + ys[i]*tau0;
    }

    ret
}