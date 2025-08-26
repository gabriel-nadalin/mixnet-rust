use rand::random_range;

use crate::traits::{Group, GroupElement};
use crate::utils::{modmul, modexp, modinv};

#[derive(Clone, PartialEq, Debug)]
pub struct U32ModElement {
    pub value: u32,
    pub modulo: u32,
}

#[derive(Debug)]
pub struct U32ModGroup {
    pub modulus: u32,
}

impl Group for U32ModGroup {
    type Elem = U32ModElement;

    fn identity(&self) -> Self::Elem {
        U32ModElement {
            value: 1,
            modulo: self.modulus,
        }
    }

    fn random_element(&self) -> Self::Elem {
        U32ModElement {
            value: random_range(0..self.modulus),
            modulo: self.modulus,
        }
    }
}

impl GroupElement for U32ModElement {
    type G = U32ModGroup;

    fn combine(&self, other: &Self) -> Self {
        assert_eq!(self.modulo, other.modulo, "Elements must be from the same group");
        U32ModElement {
            value: modmul(self.value, other.value, self.modulo),
            modulo: self.modulo,
        }
    }

    fn repeat(&self, exp: u64) -> Self {
        U32ModElement {
            value: modexp(self.value, exp as u32, self.modulo),
            modulo: self.modulo,
        }
    }

    fn inverse(&self) -> Self {
        let inv = modinv(self.value, self.modulo)
            .expect("No modular inverse exists for this value");
        U32ModElement {
            value: inv,
            modulo: self.modulo,
        }
    }
}

impl U32ModGroup {
    pub fn new(modulus: u32) -> Self {
        U32ModGroup { modulus }
    }
}