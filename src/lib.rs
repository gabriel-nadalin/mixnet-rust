pub mod utils;
pub mod el_gamal;
pub mod shuffler;
pub mod verifier;

pub const N: usize = 10;

type Ciphertext = (u32, u32);
type Proof = ((u32, u32, u32, (u32, u32), [u32; 10]), (u32, u32, u32, u32, [u32; 10], [u32; 10]), [u32; 10], [u32; 10]) ;