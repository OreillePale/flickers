use crate::api::MsGenerationSeed;

pub fn generate_ms(n: usize, mmax: usize, seed: &MsGenerationSeed) -> Vec<usize>{
    match seed{
        MsGenerationSeed::All => {
            let mut ms = Vec::new();
            for i in 1..mmax+1{
                ms.push(i);
            }

            ms
        },
        MsGenerationSeed::Explicit(mms) => {
            let mut ms = Vec::new();
            for m in mms{
                if *m <= mmax{
                    ms.push(*m);
                }
            }

            ms
        },
        MsGenerationSeed::Octave => {
            let mut ms = Vec::new();
            let mut m: usize = 1;

            while m <= mmax{
                ms.push(m);
                m *= 2;
            }

            ms
        },
        _ => panic!("{:?} is not implemented yet", seed)
    }
}