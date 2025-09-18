use std::collections::HashMap;

pub struct Vote {
    pub contest: String,
    pub choice: String,
}

impl Vote {
    pub fn encode(&self) {
        todo!()
    }
}

#[derive(Eq, Hash, PartialEq)]
pub struct TrackingCode {
    code: String,
}

impl TrackingCode {
    pub fn new() -> Self {
        todo!()
    }
}

pub struct Signature {

}

pub struct Proofs {

}

pub struct RDVEntry {
    pub tracking_code: TrackingCode,
    pub vote: Vote,
    pub time: Time,
}

pub struct RDV {
    entries: HashMap<TrackingCode, RDVEntry>,
}

impl RDV {
    pub fn new() -> Self {
        Self { entries: HashMap::new() }
    }

    pub fn add_vote(&mut self, code: TrackingCode, entry: RDVEntry) {
        self.entries.insert(code, entry);
    }
}

pub struct VoteOutput {

}