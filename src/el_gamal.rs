use crate::{utils::*, Ciphertext};
use rand::random_range;

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

    pub fn pk(&self) -> u32 {
        self.pk
    }

    pub fn keygen(&mut self) {
        self.sk = random_range(0..self.q);
        self.pk = modexp(self.g, self.sk, self.p);
    }

    pub fn encrypt(&self, m: u32) -> Ciphertext {
        let r = random_range(0..self.q);
        let c1 = modmul(
            m,
            modexp(self.pk, r, self.p),
            self.p
        );
        let c2 = modexp(self.g, r, self.p);
        return (c1, c2)
    }

    pub fn re_encrypt(&self, e: Ciphertext) -> Ciphertext {
        self.multiply_ciphertexts(e, self.encrypt(1))
    }

    pub fn decrypt(&self, ciphertext: Ciphertext) -> u32 {
        let (c1, c2) = ciphertext;
        return modmul(
            c1,
            modinv(
                modexp(c2, self.sk, self.p),
                self.p
            ).unwrap(),
            self.p
        )
    }

    pub fn multiply_ciphertexts(&self, c: Ciphertext, d: Ciphertext) -> Ciphertext {
        let (c1, c2) = c;
        let (d1, d2) = d;
        let r1 = modmul(c1, d1, self.p);
        let r2 = modmul(c2, d2, self.p);
        return (r1, r2)
    }
}