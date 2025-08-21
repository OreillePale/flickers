use crate::enums::{*};
use crate::dev_result::{*};
use crate::dev::{*};

pub struct DevComputer<'a>{
    phases: Option<&'a [f64]>,
    tau0: f64,
    dev_type: DevType,
    afs: Afs,
    noise_id: NoiseId
}

impl<'a> DevComputer<'a>{
    pub fn default() -> DevComputer<'a>{
        DevComputer{
            phases: None,
            tau0: 1.,
            dev_type: DevType::Oadev,
            afs: Afs::All,
            noise_id: NoiseId::Default
        }
    }

    pub fn compute(&self)  -> DevResult{
        let engine: Box<dyn DevEngine> = match self.dev_type{
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
}