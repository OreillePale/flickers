use crate::enums::Afs;

pub fn generate_ms(n: usize, mmax: usize, afs: &Afs) -> Vec<usize>{
    match afs{
        Afs::All => {
            let mut ms = Vec::new();
            for i in 1..mmax+1{
                ms.push(i);
            }

            ms
        },
        Afs::Explicit(mms) => {
            let mut ms = Vec::new();
            for m in mms{
                if *m <= mmax{
                    ms.push(*m);
                }
            }

            ms
        },
        Afs::Octave => {
            let mut ms = Vec::new();
            let mut m: usize = 1;

            while m <= mmax{
                ms.push(m);
                m *= 2;
            }

            ms
        },
        Afs::Decade => {
            let mut ms = Vec::new();
            let mut m: usize = 1;
            let mut i: usize = 0;
            let noms = [2,2,5];
            let denoms = [1,1,2];

            while m <= mmax{
                ms.push(m);

                m *= noms[i%noms.len()];
                m /= denoms[i%denoms.len()];
            }

            ms
        },
        _ => panic!("{:?} is not implemented yet", afs)
    }
}

//TODO: unit tests