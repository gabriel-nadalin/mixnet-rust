use rand::random_range;
use core::array::from_fn;
use crate::{Ciphertext, Proof, N, Number, NumberNZ, ModNumber, ModNumberParams, utils::{get_random, prod, hash, modnumber_to_number, ciphertext_to_number}};

pub struct Shuffler {
    p: ModNumberParams,
    q: NumberNZ,
    g: ModNumber,
    h_list: [ModNumber; N],
    pk: ModNumber
}

impl Shuffler {
    pub fn new(p: ModNumberParams, q: NumberNZ, g: ModNumber, h_list: [ModNumber; N], pk: ModNumber) -> Self {
        Self {
            p,
            q,
            g,
            h_list,
            pk
        }
    }

    pub fn gen_permutation() -> [usize; N] {
        let mut i_aux: [usize; N] = from_fn(|i| i);
        let mut psi: [usize; N] = [0; N];

        for i in 0..N {
            let k = random_range(i..N);
            psi[i] = i_aux[k];
            i_aux[k] = i_aux[i];
        }

        // println!("psi = {:?}", psi);
        // psi = [2, 0, 1];

        return psi
    }

    pub fn gen_shuffle(&self, e_list: [Ciphertext; N]) -> ([Ciphertext; N], [Number; N], [usize; N]) {
        let mut e_prime_list = [(ModNumber::zero(self.p), ModNumber::zero(self.p)); N];
        let mut e_prime_tmp = [(ModNumber::zero(self.p), ModNumber::zero(self.p)); N];
        let mut r_prime_list = [Number::ZERO; N];
        let psi = Self::gen_permutation();

        for i in 0..N {
            let (a, b) = e_list[i];

            let r_prime = get_random(&self.q).unwrap();
            let a_prime = a.mul(&self.pk.pow(&r_prime));
            let b_prime = b.mul(&self.g.pow(&r_prime));
            let e_prime = (a_prime, b_prime);

            e_prime_tmp[i] = e_prime;
            r_prime_list[i] = r_prime;
        }

        for i in 0..N {
            e_prime_list[i] = e_prime_tmp[psi[i]];
        }

        
        // println!("e_prime_list = {:?}", e_prime_list);
        // println!("r_prime_list = {:?}", r_prime_list);
        // e_prime_list = [(1578266218, 1916620270), (360773426, 1178012281), (2058523096, 973473543)];
        // r_prime_list = [516560446, 500833335, 910637767];

        return (e_prime_list, r_prime_list, psi)
    }

    pub fn gen_commitment(&self, psi: [usize; N]) -> ([ModNumber; N], [Number; N]) {
        let mut r_list = [Number::ZERO; N];
        let mut c_list = [ModNumber::zero(self.p); N];

        for i in 0..N {
            let r = get_random(&self.q).unwrap();
            let c = self.h_list[i].mul(&self.g.pow(&r));
            r_list[psi[i]] = r;
            c_list[psi[i]] = c;
        }

        // println!("r_list = {:?}", r_list);
        // println!("c_list = {:?}", c_list);
        // r_list = [110642671, 387776617, 582817700];
        // c_list = [1266380428, 988165380, 984017313];
        
        return (c_list, r_list)
    }

    pub fn gen_commitment_chain(&self, c0: ModNumber, u_list: [Number; N]) -> ([ModNumber; N], [Number; N]) {
        let mut r_list = [Number::ZERO; N];
        let mut c_list = [ModNumber::zero(self.p); N];

        for i in 0..N {
            let r = get_random(&self.q).unwrap();
            let c = self.g.pow(&r).mul(&(if i == 0 {c0} else {c_list[i-1]}).pow(&u_list[i]));
            r_list[i] = r;
            c_list[i] = c;
        }

        // println!("r_list = {:?}", r_list);
        // println!("c_list = {:?}", c_list);
        // r_list = [943942851, 295173641, 177345215];
        // c_list = [885028112, 247338767, 1093213663];

        return (c_list, r_list)
    }

    pub fn gen_proof(&self, e_list: [Ciphertext; N], e_prime_list: [Ciphertext; N], r_prime_list: [Number; N], psi: [usize; N]) -> Proof {
        let (c_list, r_list) = self.gen_commitment(psi);
        let mut u_list = [Number::ZERO; N]; // u64

        for i in 0..N {
            u_list[i] = hash(((e_list[i].0.retrieve(), e_list[i].1.retrieve(), e_prime_list[i].0.retrieve(), e_prime_list[i].1.retrieve(), c_list[i].retrieve()), i));
        }
        println!("us: {:?}", u_list);
        let u_prime_list: [Number; N] = from_fn(|i| u_list[psi[i]]);

        let (c_hat_list, r_hat_list) = self.gen_commitment_chain(self.h_list[0], u_prime_list);

        let r_bar = Number::ZERO;
        for i in 0..N {
            r_bar.add_mod(&r_list[i], &self.q);
        }

        let mut v_list = [Number::ZERO; N];
        v_list[N - 1] = Number::ONE;
        for i in (0..N-1).rev() {
            v_list[i] = v_list[i+1].mul_mod(&u_prime_list[i+1], &self.q);
        }

        let r_hat = Number::ZERO;
        let r_tilde = Number::ZERO; 
        let r_prime = Number::ZERO;
        for i in 0..N {
            r_hat.add_mod(&r_hat_list[i].mul_mod(&v_list[i], &self.q), &self.q);
            r_tilde.add_mod(&r_list[i].mul_mod(&u_list[i], &self.q), &self.q);
            r_prime.add_mod(&r_prime_list[i].mul_mod(&u_list[i], &self.q), &self.q);
        }

        let w_list: [Number; 4] = from_fn(|_| get_random(&self.q).unwrap());
        let w_hat_list: [Number; N] = from_fn(|_| get_random(&self.q).unwrap());
        let w_prime_list: [Number; N] = from_fn(|_| get_random(&self.q).unwrap());

        // println!("w_list = {:?}", w_list);
        // println!("w_hat_list = {:?}", w_hat_list);
        // println!("w_prime_list = {:?}", w_prime_list);
        // let w_list = [444114685, 983386847, 837519233, 396362824];
        // let w_hat_list = [150625670, 704677671, 787117332];
        // let w_prime_list = [302079174, 251299983, 1020180444];

        let t0 = self.g.pow(&w_list[0]);
        let t1 = self.g.pow(&w_list[1]);
        let t2 = self.g.pow(&w_list[2]).mul(&prod(from_fn(|i| self.h_list[i].pow(&w_prime_list[i])), self.p));
        let t3_0 = self.pk.pow(&w_list[3]).inv().unwrap().mul(&prod(from_fn(|i| e_prime_list[i].0.pow(&w_prime_list[i])), self.p));
        let t3_1 = self.g.pow(&w_list[3]).inv().unwrap().mul(&prod(from_fn(|i| e_prime_list[i].1.pow(&w_prime_list[i])), self.p));
        // println!("teste: {}-{}-{:?}-{:?}", self.g, w_list[3], e_prime_list, w_prime_list);

        let mut t_hat_list = [ModNumber::zero(self.p); N];
        for i in 0..N {
            t_hat_list[i] = self.g.pow(&w_hat_list[i]).mul(&(if i == 0 {self.h_list[0]} else {c_hat_list[i-1]}).pow(&w_prime_list[i]));
        }
        println!("t^[0] = {:?}", t_hat_list[0]);

        let y = (ciphertext_to_number(e_list), ciphertext_to_number(e_prime_list), modnumber_to_number(c_list), modnumber_to_number(c_hat_list), self.pk.retrieve());
        let t = (t0, t1, t2, (t3_0, t3_1), t_hat_list);
        let temp_t = (t0.retrieve(), t1.retrieve(), t2.retrieve(), (t3_0.retrieve(), t3_1.retrieve()), modnumber_to_number(t_hat_list));
        let c = hash((y, temp_t));
        println!("cs: {:?}", c);

        let s0 = w_list[0].add_mod(&c.mul_mod(&r_bar, &self.q), &self.q);
        let s1 = w_list[1].add_mod(&c.mul_mod(&r_hat, &self.q), &self.q);
        let s2 = w_list[2].add_mod(&c.mul_mod(&r_tilde, &self.q), &self.q);
        let s3 = w_list[3].add_mod(&c.mul_mod(&r_prime, &self.q), &self.q);

        let mut s_hat_list = [Number::ZERO; N];
        let mut s_prime_list = [Number::ZERO; N];
        for i in 0..N {
            s_hat_list[i] = w_hat_list[i].add_mod(&c.mul_mod(&r_hat_list[i], &self.q), &self.q);
            s_prime_list[i] =  w_prime_list[i].add_mod(&c.mul_mod(&u_prime_list[i], &self.q), &self.q);
        }
        let s = (s0, s1, s2, s3, s_hat_list, s_prime_list);
        // println!("ts    = {:?}", t);
        // println!("ss    = {:?}", s);
        return (t, s, c_list, c_hat_list)
    }
}
