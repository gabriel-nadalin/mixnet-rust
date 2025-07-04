use rand::random_range;
use sha2::{Digest, Sha512};

pub fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    let limit = (n as f32).sqrt() as u32 + 1;
    for i in (3..limit).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

pub fn safe_prime(size: u32) -> Option<(u32, u32)> {
    loop {
        let q = random_range(0..size);
        let p = 2 * q + 1;

        if is_prime(p) && is_prime(q) {
            return Some((p, q))
        }
    }
}

pub fn modmul(a: u32, b: u32, modulus: u32) -> u32 {
    ((a as u64 * b as u64) % modulus as u64) as u32
}

pub fn modinv(a: u32, modulus: u32) -> Option<u32> {
    let mut t = 0;
    let mut new_t = 1;
    let mut r = modulus as i64;
    let mut new_r = a as i64;

    while new_r != 0 {
        let q = r / new_r;
        (t, new_t) = (new_t, t - q * new_t);
        (r, new_r) = (new_r, r - q * new_r);
    }

    if r > 1 {
        return None
    }
    if t < 0 {
        t = t + modulus as i64;
    }
    return Some(t as u32)
}

pub fn modexp(base: u32, mut exp: u32, modulus: u32) -> u32 {
    if modulus == 1 {
        return 0
    }
    let mut b = base as u64;
    let m = modulus as u64;
    let mut result = 1;

    b %= m;
    while exp > 0 {
        if exp % 2 == 1 {
            result = result * b % m;
        }
        b = b * b % m;
        exp /= 2;
    }
    result as u32
}

pub fn hash(text: &str) -> u32 {
    let digest = Sha512::digest(text.as_bytes());

    // taking 4 leftmost bytes from hash; not ideal, can be solved using BigInt
    let mut bytes = [0u8; 4];
    bytes.copy_from_slice(&digest[0..4]);
    return u32::from_be_bytes(bytes)
}