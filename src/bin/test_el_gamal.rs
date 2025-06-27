use mixnet_rust::{el_gamal::ElGamal, utils::*};
use rand::random_range;

fn main() {
    let (p, q) = safe_prime(2_u32.pow(31)).unwrap();
    let mut g = random_range(0..p);
    g = modexp(g, 2, p);

    let mut el_gamal = ElGamal::new(p, q, g);
    el_gamal.keygen();

    let mut m1 = random_range(0..p);
    m1 = modexp(m1, 2, p);
    let ciphertext1 = el_gamal.encrypt(m1);
    let dec1 = el_gamal.decrypt(ciphertext1);

    let mut m2 = random_range(0..p);
    m2 = modexp(m2, 2, p);
    let ciphertext2 = el_gamal.encrypt(m2);
    let dec2 = el_gamal.decrypt(ciphertext2);

    let combined = el_gamal.multiply_ciphertexts(ciphertext1, ciphertext2);

    let decrypted = el_gamal.decrypt(combined);

    println!("message 1: {m1}");
    println!("encrypted 1: {:?}", ciphertext1);
    println!("decrypted 1: {dec1}");
    println!("message 2: {m2}");
    println!("encrypted 2: {:?}", ciphertext2);
    println!("decrypted 2: {dec2}");
    println!("combined: {:?}", combined);
    println!("combined decrypted: {decrypted}");
}
