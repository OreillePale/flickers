use flickers::{*};

fn main(){
    // generate phase data
    let phases = test_suite::generate_phase();
    let tau0 = 1.;

    let result = DevComputer::default()
        .with_phases(&phases)
        .with_tau0(1.)
        .compute();

    println!("{:?}",result);
}