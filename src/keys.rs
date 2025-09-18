use crate::{groups::{Element, Group}};

pub struct SignatureKeys<G: Group> {
    pub sk: SigningKey<G>,
    pub vk: VerificationKey<G>,
}

pub struct EncryptionKeys<G: Group> {
    pub sk: SecretKey<G>,
    pub pk: PublicKey<G>,
}

pub struct SecretKey<G: Group> {
    pub scalar: G::Scalar,
}

#[derive(Debug)]
pub struct PublicKey<G: Group> {
    pub element: G::Element,
}

pub struct SigningKey<G: Group> {
    pub scalar: G::Scalar,
}

pub struct VerificationKey<G: Group> {
    pub element: G::Element,
}

impl<G: Group> EncryptionKeys<G> {
    pub fn encrypt(&self, m: &G::Element, r: &G::Scalar) -> (G::Element, G::Element) {
        self.pk.encrypt(m, r)
    }

    pub fn decrypt(&self, c: &(G::Element, G::Element)) -> G::Element {
        self.sk.decrypt(c)
    }
}

impl<G: Group> SignatureKeys<G> {
    pub fn sign(&self, m: &G::Element) -> String {
        self.sk.sign(m)
    }

    pub fn verify(&self, m: &str) -> bool {
        self.vk.verify(m)
    }
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

impl<G: Group> PublicKey<G> {
    pub fn encrypt(&self, m: &G::Element, r: &G::Scalar) -> (G::Element, G::Element) {
        let c1 = m.add(&self.element.mul_scalar(&r));
        let c2 = self.element.group().mul_generator(&r);
        (c1, c2)
    }
}

impl<G: Group> SigningKey<G> {
    pub fn sign(&self, m: &G::Element) -> String {
        todo!()
    }
}

impl<G: Group> VerificationKey<G> {
    pub fn verify(&self, m: &str) -> bool {
        todo!()
    }
}

pub fn keygen<G: Group>() -> (EncryptionKeys<G>, SignatureKeys<G>) {
    todo!()
}