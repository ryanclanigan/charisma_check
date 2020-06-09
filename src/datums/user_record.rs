use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Eq, PartialEq, Debug)]
pub struct UserRecord {
    pub id: u64,
    pub score: u64,
}
