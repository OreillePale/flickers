pub mod adev;
pub mod oadev;

use crate::noise::detect::lag1::lag1_single;
use crate::noise::detect::NoiseDetectionStrategy;
use crate::noise::detect::noise_id;

// TODO: move elsewhere
enum DevType{
    Adev,
    Oadev,
}

// TODO: add Octave and Decade
// TODO: move elsewhere
enum MsGenerationSeed{
    All,
    Explicit(Vec<usize>)
}

pub trait DevEngine{

    const PREFERRED_NOISE_ID_STRATEGY: NoiseDetectionStrategy;

    fn name() -> &'static str;

    fn compute_one(&self, xs: &[f64], m: usize, tau0: f64) -> f64;

    fn compute_many(&self, xs: &[f64], ms: &[usize], tau0: f64) -> Vec<f64>{
        ms.iter().map(|m| self.compute_one(&xs,*m, tau0)).collect()
    }

    fn m_max(&self, n: usize) -> usize;

    fn generate_ms(&self, n: usize, seed: &MsGenerationSeed) -> Vec<usize>{
        match seed{
            MsGenerationSeed::All => {
                let mut ms = Vec::new();
                for i in 1..self.m_max(n)+1{
                    ms.push(i);
                }

                ms
            },
            MsGenerationSeed::Explicit(mms) => {
                let mmax = self.m_max(n);
                let mut ms = Vec::new();
                for m in mms{
                    if *m <= mmax{
                        ms.push(*m);
                    }
                }

                ms
            }
        }
    }

    // TODO: add parallel mode
    fn compute(&self, xs: &[f64], tau0: f64, seed: &MsGenerationSeed) -> DevResult{
        // compute de avering factors
        let ms = self.generate_ms(xs.len(), &seed);
        let taus = ms.iter().map(|m| (*m as f64)*tau0).collect::<Vec<_>>();

        // compute the devs; could be parallelized, maybe later
        let devs = self.compute_many(&xs, &ms, tau0);

        // compute the noise types
        let alphas = ms.iter().map(|m| noise_id(&xs, *m, tau0, Self::PREFERRED_NOISE_ID_STRATEGY)).collect::<Vec<f64>>();

        // compute points used for calculating each dev
        let ns = ms.iter().map(|m| self.ns(xs.len(),*m)).collect::<Vec<usize>>();

        // compute edfs
        let edfs = ns.iter().zip(alphas.iter()).zip(ms.iter()).map(|((n,alpha),m)| self.edf(*alpha,*n,*m)).collect::<Vec<f64>>();

        let err_factors = edfs.iter().zip(alphas.iter()).map(|(edf,alpha)| self.err_factor(*alpha,*edf)).collect::<Vec<_>>();

        DevResult{
            taus: taus,
            devs: devs,
            ns: ns,
            alphas: alphas,
            edfs: edfs,
            err_factors: err_factors,
        }
    }

    // number of points used to calculate the dev
    fn ns(&self, n: usize, m: usize) -> usize;

    fn edf(&self, alpha: f64, n: usize, m: usize) -> f64;

    fn err_factor(&self, alpha: f64, edf: f64) -> f64;
}

struct DevResult{
    taus: Vec<f64>,
    devs: Vec<f64>,
    ns: Vec<usize>,
    alphas: Vec<f64>,
    edfs: Vec<f64>,
    err_factors: Vec<f64>
}