use crate::{groups::{Element, Group}};

pub struct SignatureKeys {
    
}

pub struct SecretKey<G: Group> {
    pub scalar: G::Scalar,
}

impl<G: Group> SecretKey<G> {
    pub fn new(group: &G) -> Self {
        Self {
            scalar: group.random_scalar()
        }
    }

    pub fn decrypt(&self, c: &(G::Element, G::Element)) -> G::Element {
        let (c1, c2) = c;
        c1.add(&c2.mul_scalar(&self.scalar).inv())
    }

    pub fn public_key(&self, group: &G) -> PublicKey<G> {
        PublicKey {
            element: group.mul_generator(&self.scalar)
        }
    }
}


#[derive(Debug)]
pub struct PublicKey<G: Group> {
    pub element: G::Element,
}

impl<G: Group> PublicKey<G> {
    pub fn encrypt(&self, m: &G::Element, r: &G::Scalar) -> (G::Element, G::Element) {
        let c1 = m.add(&self.element.mul_scalar(&r));
        let c2 = self.element.group().mul_generator(&r);
        (c1, c2)
    }
}

pub fn keygen<G: Group>() -> (SecretKey<G>, PublicKey<G>) {
    todo!()
}