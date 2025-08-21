#[derive(Clone,Debug)]
pub enum DevType{
    Adev,
    Oadev
}

// that is a great start already
#[derive(Clone,Debug)]
pub enum Afs{
    All,
    Decade,
    Octave,
    Explicit(Vec<usize>),
    PointsPerDecade(usize),
}

#[derive(Clone,Debug)]
pub enum NoiseId{
    Default, // use default NoiseId algorithm based on each deviation
    Lag1(usize,usize),
    Lag1B1(usize,usize),
    OverlappingLag1B1,
    B1,
    Rn,
}