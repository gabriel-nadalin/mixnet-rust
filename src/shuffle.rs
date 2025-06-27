use crate::{utils::{modexp, modmul}, N};
use rand::random_range;

pub struct Shuffle {
    p: u32,
    q: u32,
    g: u32,
    h_list: [u32; N],
    pk: u32
}

impl Shuffle {
    pub fn new(p: u32, q: u32, g: u32, h_list: [u32; N], pk: u32) -> Self {
        Self {
            p,
            q,
            g,
            h_list,
            pk
        }
    }

    pub fn gen_permutation() -> [u32; N] {
        let mut i_aux: [u32; N] = core::array::from_fn(|i| i as u32);
        let mut psi: [u32; N] = [0; N];

        for i in 0..N {
            let k = random_range(i..N);
            psi[i] = i_aux[k];
            i_aux[k] = i_aux[i];
        }
        return psi
    }

    pub fn gen_shuffle(&self, e_list: [(u32, u32); N]) -> ([(u32, u32); N], [u32; N], [u32; N]) {
        let mut e_new_list = [(0, 0); N];
        let mut r_list = [0; N];
        let psi = Self::gen_permutation();

        for i in 0..N {
            let (a, b) = e_list[i];

            let r_new = random_range(0..self.q);
            let a_new = modmul(a,
                modexp(self.pk, r_new, self.p),
                self.p
            );
            let b_new = modmul(b,
                modexp(self.g, r_new, self.p),
                self.p
            );
            let e_new = (a_new, b_new);

            e_new_list[psi[i] as usize] = e_new;
            r_list[i] = r_new;
        }

        return (e_new_list, r_list, psi)
    }
}