#[cfg(feature = "python")]
use pyo3::pyclass;

#[cfg_attr(feature = "python", pyclass)]
#[cfg_attr(feature = "serialization", derive(serde::Serialize))]
#[derive(Clone,Debug)]
pub enum DevType{
    Adev,
    Oadev
}

#[cfg_attr(feature = "python", pyclass)]
#[cfg_attr(feature = "serialization", derive(serde::Serialize))]
#[derive(Clone,Debug)]
pub enum Afs{
    All(),
    Decade(),
    Octave(),
    Explicit{afs: Vec<usize>},
    PointsPerDecade{n: usize},
}

#[cfg_attr(feature = "python", pyclass)]
#[cfg_attr(feature = "serialization", derive(serde::Serialize))]
#[derive(Clone,Debug)]
pub enum NoiseId{
    Default(), // use default NoiseId algorithm based on each deviation
    Lag1{dmin:usize,dmax:usize},
    Lag1B1{dmin:usize,dmax:usize},
    B1(),
    Rn(),
}