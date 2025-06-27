use mixnet_rust::{el_gamal::ElGamal, utils::*};

fn main() {
    let (p, q) = safe_prime(0, 2_u32.pow(31)).unwrap();
    let mut g = rand::random_range(..p);
    g = modexp(g, 2, p);

    let mut el_gamal = ElGamal::new(p, q, g);
    el_gamal.keygen();

    let mut m1 = rand::random_range(..p);
    m1 = modexp(m1, 2, p);
    let ciphertext1 = el_gamal.encrypt(m1);

    let mut m2 = rand::random_range(..p);
    m2 = modexp(m2, 2, p);
    let ciphertext2 = el_gamal.encrypt(m2);

    let combined = el_gamal.multiply_ciphertexts(ciphertext1, ciphertext2);

    let decrypted = el_gamal.decrypt(combined);

    println!("message: {m1} {m2}");
    println!("encrypted 1: {:?}", ciphertext1);
    println!("encrypted 2: {:?}", ciphertext2);
    println!("combined decrypted: {decrypted}");
}
