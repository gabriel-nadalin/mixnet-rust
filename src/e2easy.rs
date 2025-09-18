use std::sync::Arc;

use crate::{groups::Group, keys::{self, EncryptionKeys, SignatureKeys}, shuffler::Shuffler, types::*};

pub struct E2Easy<G: Group> {
    group: Arc<G>,
    enc_keys: EncryptionKeys<G>,
    sig_keys: SignatureKeys<G>,
    vote_table: RDV,
    last_tracking_code: TrackingCode,
    
    // shuffler: Shuffler,
}

impl<G: Group> E2Easy<G> {
    // seria possivel combinar setup() e start() em new()?
    pub fn new(group: Arc<G>) -> Self {
        let (enc_keys, sig_keys) = keys::keygen();
        Self {
            group,
            enc_keys,
            sig_keys,
            vote_table: RDV::new(),
            last_tracking_code: TrackingCode::new(),
        }
    }

    pub fn setup() {
        todo!()
    }

    pub fn start() {
        todo!()
    }

    // qual o formato dos votos?
    // e do CR?
    pub fn vote(&self, vote: Vote) -> TrackingCode {
        let r = self.group.random_scalar();
        let enc_vote = self.enc_keys.encrypt(vote.encode(), &r);
        hash(enc_vote, timestamp, self.last_tracking_code)
    }

    pub fn challenge(&self, code: TrackingCode) -> (Hash, Vote, Nonce) {
        todo!()
    }

    pub fn cast(&self, code: TrackingCode) -> Signature {
        todo!()
    }

    pub fn tally() -> (Proofs, RDV, VoteOutput) {
        todo!()
    }

    pub fn finish() {
        todo!()
    }
}