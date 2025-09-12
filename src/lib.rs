use crypto_bigint::{Uint, NonZero, modular::{MontyForm, MontyParams}};

pub mod utils;
// pub mod traits;
// pub mod groups;
pub mod el_gamal;
pub mod shuffler;
pub mod verifier;

pub const N: usize = 10;

pub const SIZE: usize = 256; // Length in bits
pub type Number = Uint<{SIZE/64}>; // Convert bits to LIMBS
pub type NumberNZ = NonZero<Number>; // Convert bits to LIMBS
pub type ModNumber = MontyForm<{SIZE/64}>;
pub type ModNumberParams = MontyParams<{SIZE/64}>;
pub type Ciphertext = (ModNumber, ModNumber);
type Proof = ((ModNumber, ModNumber, ModNumber, (ModNumber, ModNumber), [ModNumber; N]), (Number, Number, Number, Number, [Number; N], [Number; N]), [ModNumber; N], [ModNumber; N]);
