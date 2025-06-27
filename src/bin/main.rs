use mixnet_rust::{el_gamal::ElGamal, utils::*};
use rand::random;

fn main() {
    let (p, q) = safe_prime(0, 2_u32.pow(31)).unwrap();
    let mut g = rand::random_range(..p);
    g = modexp(g, 2, p);

    let mut el_gamal = ElGamal::new(p, q, g);
    el_gamal.keygen();

    let m = 42;

    println!("message: {m}");
    let cyphertext = el_gamal.encrypt(m);
    println!("encrypted: {:?}", cyphertext);
    let dec = el_gamal.decrypt(cyphertext);
    println!("decrypted: {dec}");
}
