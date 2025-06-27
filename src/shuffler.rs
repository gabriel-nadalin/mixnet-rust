use crate::{utils::{hash, modexp, modinv, modmul}, N};
use rand::random_range;

pub struct Shuffler {
    p: u32,
    q: u32,
    g: u32,
    h_list: [u32; N],
    pk: u32
}

impl Shuffler {
    pub fn new(p: u32, q: u32, g: u32, h_list: [u32; N], pk: u32) -> Self {
        Self {
            p,
            q,
            g,
            h_list,
            pk
        }
    }

    pub fn gen_permutation() -> [usize; N] {
        let mut i_aux: [usize; N] = core::array::from_fn(|i| i);
        let mut psi: [usize; N] = [0; N];

        for i in 0..N {
            let k = random_range(i..N);
            psi[i] = i_aux[k];
            i_aux[k] = i_aux[i];
        }
        return psi
    }

    pub fn gen_shuffle(&self, e_list: [(u32, u32); N]) -> ([(u32, u32); N], [u32; N], [usize; N]) {
        let mut e_prime_list = [(0, 0); N];
        let mut r_list = [0; N];
        let psi = Self::gen_permutation();

        for i in 0..N {
            let (a, b) = e_list[i];

            let r_prime = random_range(0..self.q);
            let a_prime = modmul(a,
                modexp(self.pk, r_prime, self.p),
                self.p
            );
            let b_prime = modmul(b,
                modexp(self.g, r_prime, self.p),
                self.p
            );
            let e_prime = (a_prime, b_prime);

            e_prime_list[psi[i]] = e_prime;
            r_list[i] = r_prime;
        }

        return (e_prime_list, r_list, psi)
    }

    pub fn gen_commitment(&self, psi: [usize; N]) -> ([u32; N], [u32; N]) {
        let mut r_list = [0; N];
        let mut c_list = [0; N];

        for i in 0..N {
            let r = random_range(0..self.q);
            let c = modmul(
                modexp(self.g, r, self.p),
                self.h_list[i],
                self.p
            );
            r_list[psi[i]] = r;
            c_list[psi[i]] = c;
        }
        return (c_list, r_list)
    }

    pub fn gen_commitment_chain(&self, c0: u32, u_list: [u32; N]) -> ([u32; N], [u32; N]) {
        let mut r_list = [0; N];
        let mut c_list = [0; N];

        for i in 0..N {
            let r = random_range(0..self.q);
            let c = modmul(
                modexp(self.g, r, self.p),
                modexp(if i == 0 {c0} else {c_list[i-1]}, u_list[i], self.p),
                self.p
            );
            r_list[i] = r;
            c_list[i] = c;
        }
        return (c_list, r_list)
    }

    pub fn gen_proof(
        &self,
        e_list: [(u32, u32); N],
        e_prime_list: [(u32, u32); N],
        r_prime_list: [u32; N],
        psi: [usize; N]
    ) -> (
        (u32, u32, u32, (u32, u32), [u32; 10]),
        (u32, u32, u32, u32, [u32; 10], [u32; 10]),
        u32,
        [u32; 10]
    ) {
        let (c_list, r_list) = self.gen_commitment(psi);
        let mut u_list = [0; N];

        for i in 0..N {
            let statement = format!("{:?}{:?}{:?}{}", e_list, e_prime_list, c_list, i);
            let u = hash(&statement);

            u_list[i] = u;
        }
        let u_prime_list: [u32; N] = core::array::from_fn(|i| u_list[psi[i]]);

        let (c_hat_list, r_hat_list) = self.gen_commitment_chain(self.h_list[0], u_prime_list);

        let mut r_bar = 0;
        for i in 0..N {
            r_bar = (r_bar + r_list[i]) % self.q;
        }

        let mut v_list = [0; N];
        v_list[N - 1] = 1;
        for i in (0..N-1).rev() {
            v_list[i] = modmul(u_prime_list[i+1], v_list[i+1], self.q);
        }

        let mut r_hat = 0;
        let mut r_tilde = 0;
        let mut r_prime = 0;
        for i in 0..N {
            r_hat = (r_hat + modmul(r_hat_list[i], v_list[i], self.q)) % self.q;
            r_tilde = (r_tilde + modmul(r_list[i], u_list[i], self.q)) % self.q;
            r_prime = (r_prime + modmul(r_prime_list[i], u_list[i], self.q)) % self.q;
        }

        let w_list: [u32; 4] = core::array::from_fn(|_| random_range(0..self.q));
        let w_hat_list: [u32; N] = core::array::from_fn(|_| random_range(0..self.q));
        let w_prime_list: [u32; N] = core::array::from_fn(|_| random_range(0..self.q));

        let t0 = modexp(self.g, w_list[0], self.p);
        let t1 = modexp(self.g, w_list[1], self.p);
        let mut t2 = modexp(self.g, w_list[2], self.p);
        for i in 0..N {
            t2 = modmul(
                t2,
                modexp(self.h_list[i], w_prime_list[i], self.p),
                self.p
            );
        }
        let mut t3_0 = modinv(modexp(self.pk, w_list[3], self.p), self.p).unwrap();
        let mut t3_1 = modinv(modexp(self.g, w_list[3], self.p), self.p).unwrap();
        for i in 0..N {
            t3_0 = modmul(t3_0, modexp(e_prime_list[i].0, w_prime_list[i], self.p), self.p);
            t3_1 = modmul(t3_1, modexp(e_prime_list[i].1, w_prime_list[i], self.p), self.p);
        }

        let c_hat_0 = self.h_list[0];
        let mut t_hat_list = [0; N];
        for i in 0..N {
            t_hat_list[i] = modmul(
                modexp(self.g, w_hat_list[i], self.p),
                modmul(if i == 0 {c_hat_0} else {c_hat_list[i-1]}, w_prime_list[i], self.p),
                self.p
            );
        }

        let y = (e_list, e_prime_list, c_list, c_hat_list, self.pk);
        let t = (t0, t1, t2, (t3_0, t3_1), t_hat_list);
        let c = hash(&format!("{:?}{:?}", y, t));

        let s0 = (w_list[0] + modmul(c, r_bar, self.q)) % self.q;
        let s1 = (w_list[1] + modmul(c, r_hat, self.q)) % self.q;
        let s2 = (w_list[2] + modmul(c, r_tilde, self.q)) % self.q;
        let s3 = (w_list[3] + modmul(c, r_prime, self.q)) % self.q;

        let mut s_hat_list = [0; N];
        let mut s_prime_list = [0; N];
        for i in 0..N {
            s_hat_list[i] = (w_hat_list[i] + modmul(c, r_hat_list[i], self.q)) % self.q;
            s_prime_list[i] = (w_prime_list[i] + modmul(c, u_prime_list[i], self.q)) % self.q;
        }
        let s = (s0, s1, s2, s3, s_hat_list, s_prime_list);

        return (t, s, c, c_hat_list)
    }
}