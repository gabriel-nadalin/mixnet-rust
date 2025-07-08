use crate::{utils::*, *};
use core::array::from_fn;

pub struct Verifier {
    p: u32,
    q: u32,
    g: u32,
    h_list: [u32; N]
}

impl Verifier {
    pub fn new(p: u32, q: u32, g: u32, h_list: [u32; N]) -> Self {
        Self {
            p,
            q,
            g,
            h_list
        }
    }

    pub fn check_proof(&self, pi: Proof, e_list: [Ciphertext; N], e_prime_list: [Ciphertext; N], pk: u32) -> bool {
        let (t, s, c_list, c_hat_list) = pi;

        let mut u_list = [0; N];
        for i in 0..N {
            u_list[i] = hash(((e_list, e_prime_list, c_list), i), self.q);
        }
        // println!("uv: {:?}", u_list);

        let c_bar = modmul(
            prod(c_list, self.p),
            modinv(prod(self.h_list, self.p), self.p).unwrap(),
            self.p
        );
        let u = prod(u_list, self.q);

        let c_hat = modmul(
            c_hat_list[N-1],
            modinv(
                modexp(self.h_list[0], u, self.p),
                self.p
            ).unwrap(),
            self.p
        );

        let c_tilde = prod(
            from_fn(|i| modexp(c_list[i], u_list[i], self.p)),
            self.p
        );
        let a_prime = prod(
            from_fn(|i| modexp(e_list[i].0, u_list[i], self.p)),
            self.p
        );
        let b_prime = prod(
            from_fn(|i| modexp(e_list[i].1, u_list[i], self.p)),
            self.p
        );

        let y = (e_list, e_prime_list, c_list, c_hat_list, pk);
        let c = hash((y, t), self.q);
        // println!("cv: {:?}", c);

        let t_prime_0 = modmul(
            modinv(
                modexp(c_bar, c, self.p),
                self.p
            ).unwrap(),
            modexp(self.g, s.0, self.p),
            self.p
        );
        let t_prime_1 = modmul(
            modinv(
                modexp(c_hat, c, self.p),
                self.p
            ).unwrap(),
            modexp(self.g, s.1, self.p),
            self.p
        );
        let t_prime_2 = modmul(
            modmul(
                modinv(
                    modexp(c_tilde, c, self.p),
                    self.p
                ).unwrap(),
                modexp(self.g, s.2, self.p),
                self.p
            ),
            prod(
                from_fn(|i| modexp(self.h_list[i], s.5[i], self.p)),
                self.p
            ),
            self.p
        );
        let t_prime_3_0 = modmul(
            modmul(
                modinv(
                    modexp(a_prime, c, self.p),
                    self.p
                ).unwrap(),
                modinv(
                    modexp(pk, s.3, self.p),
                    self.p
                ).unwrap(),
                self.p
            ),
            prod(
                from_fn(|i| modexp(e_prime_list[i].0, s.5[i], self.p)),
                self.p
            ),
            self.p
        );
        let t_prime_3_1 = modmul(
            modmul(
                modinv(
                    modexp(b_prime, c, self.p),
                    self.p
                ).unwrap(),
                modinv(
                    modexp(self.g, s.3, self.p),
                    self.p
                ).unwrap(),
                self.p
            ),
            prod(
                from_fn(|i| modexp(e_prime_list[i].1, s.5[i], self.p)),
                self.p
            ),
            self.p
        );

        let mut t_hat_prime_list = [0; N];
        for i in 0..N {
            t_hat_prime_list[i] = modmul(
                modmul(
                    modinv(
                        modexp(c_hat_list[i], c, self.p),
                        self.p
                    ).unwrap(),
                    modexp(self.g, s.4[i], self.p),
                    self.p
                ),
                modexp(
                    if i == 0 {self.h_list[0]} else {c_hat_list[i-1]},
                    s.5[i],
                    self.p
                ),
                self.p
            )
        }

        let t_prime = (t_prime_0, t_prime_1, t_prime_2, (t_prime_3_0, t_prime_3_1), t_hat_prime_list);
        // println!("s     = {:?}", s);
        println!("t     = {:?}", t);
        println!("t'    = {:?}", t_prime);
        return t == t_prime
    }
}