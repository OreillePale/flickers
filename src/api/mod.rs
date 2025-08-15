pub mod dev_result;

use crate::dev::{adev,oadev,DevEngine};
use crate::api::dev_result::{*};
use crate::test_suite::{*}; // user can call the test suite

#[derive(Debug)]
pub enum MsGenerationSeed{
    All,
    Explicit(Vec<usize>),
    Octave,
    Decade,
    PointsPerDecade(usize),
}

#[derive(Debug)]
pub enum DataType{
    Phase,
    Frequency
}

#[derive(Debug)]
pub enum DevType{
    Adev,
    Oadev
}

fn get_dev_engine(dev_type: DevType) -> Box<dyn DevEngine>{
    match dev_type{
        DevType::Adev => Box::new(adev::AdevEngine::new()),
        DevType::Oadev => Box::new(oadev::OadevEngine::new()),
        _ => panic!("Deviation {:?} is not supported (yet)", dev_type)
    }
}

pub fn compute_dev(dev_type: DevType, data: &[f64], tau0: f64, data_type: DataType, ms: MsGenerationSeed) -> DevResult{
    // handle data type
    let phases = match data_type{
        DataType::Phase => data,
        _ => panic!("Data type handling {:?} is not implemented yet", data_type)
    };
    
    // handle engine
    let engine = get_dev_engine(dev_type);

    // compute
    let result = engine.compute(&phases, tau0, &ms);

    // return result
    result
    
}


