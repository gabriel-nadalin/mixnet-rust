use hex::ToHex;
use p256::{FieldBytes, NistP256, ProjectivePoint, elliptic_curve::{Field, Group, PrimeField, hash2curve::{ExpandMsgXmd, GroupDigest}}};
use serde::Serialize;
use crate::{Scalar, Element};
use sha2::{Digest, Sha256};
use rand_core::OsRng;

pub fn random_element() -> Element {
    return ProjectivePoint::random(&mut OsRng).to_affine()
}

pub fn random_scalar() -> Scalar {
    return Scalar::random(&mut OsRng)
}

pub fn summation (list: Vec<ProjectivePoint>) -> ProjectivePoint {
    let mut sum: ProjectivePoint = ProjectivePoint::IDENTITY;
    let n = list.len();
    for i in 0..n {
        sum += list[i];
    }
    return sum
}


// strict: only accepts canonical 32-byte scalar encoding.
pub fn scalar_from_bytes_strict(bytes: &[u8]) -> Option<Scalar> {
    if bytes.len() != 32 {
        return None;
    }

    let mut arr = [0u8; 32];
    arr.copy_from_slice(bytes);

    Scalar::from_repr(FieldBytes::from(arr)).into()
}

pub fn derive_nonces(seed: &Scalar, count: usize) -> Vec<Scalar> {
    let mut nonces = Vec::with_capacity(count);
    for i in 0..count {
        let to_hash = (seed, i);
        nonces.push(hash2scalar(&to_hash));
    }
    nonces
}

pub fn derive_h_list(seed: &str, count: usize) -> Vec<Element> {
    let mut h_list = Vec::with_capacity(count);
    for i in 0..count {
        let to_hash = (seed, i);
        h_list.push(hash2element(&to_hash));
    }
    h_list
}

/// hashes a serializable object into a hex string
pub fn hash2str<T: Serialize + ?Sized>(obj: &T) -> String {
    Sha256::digest(serde_json_canonicalizer::to_vec(&obj).unwrap()).encode_hex_upper()
}

/// hashes a serializable object into a scalar
pub fn hash2scalar<T: Serialize + ?Sized>(obj: &T) -> Scalar {
    let data = serde_json_canonicalizer::to_vec(&obj).unwrap();
    NistP256::hash_to_scalar::<ExpandMsgXmd<sha2::Sha256>>(&[&data], &[b"E2EASY-PC/NONCE/V1"]).unwrap()
}

/// hashes a serializable object into a scalar
pub fn hash2element<T: Serialize + ?Sized>(obj: &T) -> Element {
    let data = serde_json_canonicalizer::to_vec(&obj).unwrap();
    NistP256::hash_from_bytes::<ExpandMsgXmd<sha2::Sha256>>(&[&data], &[b"E2EASY-PC/HLIST/V1"]).unwrap().to_affine()
}