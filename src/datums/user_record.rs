use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Eq, PartialEq, Debug)]
pub struct UserRecord {
    pub id: u64,
    pub score: u64,
}

impl UserRecord {
    pub fn with_score(&self, score: u64) -> Self {
        UserRecord { id: self.id, score }
    }

    pub fn new(id: u64, score: u64) -> Self {
        UserRecord { id, score }
    }
}
