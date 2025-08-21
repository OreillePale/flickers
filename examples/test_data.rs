use flickers::{test_suite};

fn main(){
    // generate phase data
    let phases = test_suite::generate_phase();
    println!("{:?}",phases);
}