use rand::prelude::*;
use num_primes::Generator;

use crate::utils::*;

pub struct ElGamal {
    p: u32,
    q: u32,
    g: u32,
    sk: u32,
    pk: u32
}

impl ElGamal {
    pub fn new(group: u32, order: u32, generator: u32) -> Self {
        Self {
            p: group,
            q: order,
            g: generator,
            sk: 0,
            pk: 0,
        }
    }

    pub fn keygen(&mut self) {
        self.sk = rand::random_range(..self.q);
        self.pk = modexp(self.g, self.sk, self.p);
    }

    pub fn encrypt(&self, m: u32) -> (u32, u32) {
        let r = rand::random_range(..self.q);
        let c1 = modmul(
            m,
            modexp(self.pk, r, self.p),
            self.p
        );
        let c2 = modexp(self.g, r, self.p);
        return (c1, c2)
    }

    pub fn decrypt(&self, cyphertext: (u32, u32)) -> u32 {
        let (c1, c2) = cyphertext;
        return modmul(
            c1,
            modinv(
                modexp(c2, self.sk, self.p),
                self.p
            ).unwrap(),
            self.p
        )
    }
}