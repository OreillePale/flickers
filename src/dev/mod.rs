pub mod adev;
pub mod oadev;

use crate::noise::detect::noise_id;
use crate::enums::{*};
use crate::utils::ms;
use crate::dev_result::{*};

pub trait DevEngine{

    fn dev(&self) -> DevType;

    fn preferred_noise_id_metod(&self) -> NoiseId;

    fn compute_one(&self, xs: &[f64], m: usize, tau0: f64) -> f64;

    fn compute_many(&self, xs: &[f64], ms: &[usize], tau0: f64) -> Vec<f64>{
        ms.iter().map(|m| self.compute_one(&xs,*m, tau0)).collect()
    }

    fn m_max(&self, n: usize) -> usize;

    // TODO: add parallel mode
    fn compute(&self, xs: &[f64], tau0: f64, afs: &Afs, noise_id_method: NoiseId) -> DevResult{
        // compute the avering factors
        let ms = ms::generate_ms(xs.len(), self.m_max(xs.len()), &afs);
        let taus = ms.iter().map(|m| (*m as f64)*tau0).collect::<Vec<_>>();

        // compute the devs; could be parallelized; useful with theo1
        let devs = self.compute_many(&xs, &ms, tau0);

        // compute the noise types; maybe this can be parallelized
        let noise_id_method_final = match noise_id_method{
            NoiseId::Default() => self.preferred_noise_id_metod(),
            _ => noise_id_method.clone()
        };

        let alphas = ms.iter().map(|m| noise_id(&xs, *m, tau0, noise_id_method_final.clone())).collect::<Vec<f64>>();

        // compute points used for calculating each
        let ns = ms.iter().map(|m| self.ns(xs.len(),*m)).collect::<Vec<usize>>();

        // compute edfs
        let edfs = alphas.iter().zip(ms.iter()).map(|(alpha,m)| self.edf(*alpha,xs.len(),*m)).collect::<Vec<f64>>();

        // I think that is NOT correct
        // I should get an (sigma_min,sigma_max) pair
        let cis = edfs.iter()
            .map(|edf| self.ci_factor(*edf,0.025))
            .zip(devs.iter())
            .map(|(cif,dev)| (cif.0*dev,cif.1*dev))
            .collect::<Vec<(_,_)>>();

        DevResult::default()
            .with_dev(self.dev())
            .with_taus(taus)
            .with_devs(devs)
            .with_ns(ns)
            .with_alphas(alphas)
            .with_edfs(edfs)
            .with_cis(cis)
            .with_noise_id(noise_id_method_final)
    }

    // number of points used to calculate the dev
    fn ns(&self, n: usize, m: usize) -> usize;

    fn edf(&self, alpha: f64, n: usize, m: usize) -> f64;

    // no; the confidence interval is actually [sigma_min,sigma_max], not [err_min,err_max]; there is some mixup in stable32 documentation
    fn ci_factor(&self, edf: f64, p: f64) -> (f64,f64);
}