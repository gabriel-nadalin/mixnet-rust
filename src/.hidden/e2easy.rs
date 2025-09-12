use crate::{el_gamal::ElGamal, keys::{PublicKey, SecretKey}, shuffler::Shuffler};

pub struct E2Easy {
    // as chaves devem ser armazenadas explicitamente aqui?
    // pk: PublicKey,
    // sk: SecretKey,

    // ou contidas na struct ElGamal?
    elgamal: ElGamal,
    
    shuffler: Shuffler,
}

impl E2Easy {
    // seria possível combinar setup() e start() em new()?
    pub fn new() -> Self {
        todo!()
    }

    pub fn setup() {
        todo!()
    }

    pub fn start() {
        todo!()
    }

    pub fn vote(vote: Vote) -> TrackingCode {
        todo!()
    }

    pub fn challenge() -> (Hash, Vote, Nonce) {
        todo!()
    }

    pub fn cast() -> Signature {
        todo!()
    }

    pub fn tally() -> (Proofs, RDV, VoteOutput) {
        todo!()
    }

    pub fn finish() {
        todo!()
    }
}