use crate::enums::{*};
use crate::dev_result::{*};
use crate::dev::{*};

/// Api for the user to compute Allan-like deviations
///
/// # Example
/// Take a look to `examples/adev.rs`
///
pub struct DevComputer<'a>{
    phases: Option<&'a [f64]>,
    tau0: f64,
    dev: DevType,
    afs: Afs,
    noise_id: NoiseId
}

impl<'a> DevComputer<'a>{
    pub fn default() -> DevComputer<'a>{
        DevComputer{
            phases: None,
            tau0: 1.,
            dev: DevType::Oadev,
            afs: Afs::All(),
            noise_id: NoiseId::Default()
        }
    }

    pub fn compute(&self)  -> DevResult{
        let engine: Box<dyn DevEngine> = match self.dev{
            DevType::Adev => Box::new(adev::AdevEngine::new()),
            DevType::Oadev => Box::new(oadev::OadevEngine::new()),
        };

        engine.compute(&self.phases.unwrap(), self.tau0, &self.afs, self.noise_id.clone())
    }

    pub fn with_phases(mut self, phases: &'a [f64]) -> Self{
        self.phases = Some(phases);
        self
    }

    pub fn with_tau0(mut self, tau0: f64) -> Self{
        self.tau0 = tau0;
        self
    }

    pub fn with_dev(mut self, dev: DevType) -> Self{
        self.dev = dev;
        self
    }

    pub fn with_afs(mut self, afs: Afs) -> Self{
        self.afs = afs;
        self
    }

    pub fn with_noise_id(mut self, noise_id: NoiseId) -> Self{
        self.noise_id = noise_id;
        self
    }
}