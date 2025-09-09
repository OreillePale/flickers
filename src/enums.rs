#[cfg_attr(feature = "serialization", derive(serde::Serialize,serde::Deserialize),serde(rename_all = "lowercase"))]
#[derive(Clone,Debug)]
pub enum DevType{
    Adev,
    Oadev
}

#[cfg_attr(feature = "serialization", derive(serde::Serialize,serde::Deserialize),serde(rename_all = "lowercase"))]
#[derive(Clone,Debug)]
pub enum Afs{
    All,
    Decade,
    Octave,
    Explicit{afs: Vec<usize>},
    PointsPerDecade{n: usize},
}

// #TODO: add constant NoiseId
#[cfg_attr(feature = "serialization", derive(serde::Serialize,serde::Deserialize),serde(rename_all = "lowercase"))]
#[derive(Clone,Debug)]
pub enum NoiseId{
    Default, // use default NoiseId algorithm based on each deviation
    B1,
    Rn,
    Lag1{dmin:usize,dmax:usize},
    Lag1B1{dmin:usize,dmax:usize},
    Alpha{alpha:f64},
}