use super::user_record::UserRecord;
use crate::errors::result::Result;
use crate::serializers::serializer::Serializer;
use crate::serializers::user_serializer::UserSerializer;
use std::collections::HashMap;
use std::path::Path;

/// A container for users and their scores.
/// TODO: Replace with abstraction over users, scores, and channel IDs, fully allowing for multiple users of this
pub struct Users {
    users_to_scores: HashMap<u64, u64>,
}

impl Users {
    pub fn new() -> Self {
        Users {
            users_to_scores: HashMap::new(),
        }
    }

    pub fn put(&mut self, user: u64, score: u64, threshold: u64) -> Option<u64> {
        let result = self.users_to_scores.get(&user);
        let previous = match result {
            Some(s) => self.users_to_scores.insert(user, score + s),
            None => self.users_to_scores.insert(user, score),
        };
        match previous {
            None => None,
            Some(s) => {
                if s > threshold {
                    Some(s)
                } else {
                    None
                }
            }
        }
    }

    pub fn to_records(&self) -> Vec<UserRecord> {
        self.users_to_scores
            .iter()
            .map(|(id, score)| UserRecord {
                id: *id,
                score: *score,
            })
            .collect()
    }

    pub fn size(&self) -> usize {
        self.users_to_scores.len()
    }

    pub fn write(&mut self, path: &Path) -> Result<()> {
        match UserSerializer::new(path) {
            Err(e) => Err(e),
            Ok(serializer) => {
                let on_disk_users = match serializer.read() {
                    Err(e) => return Err(e),
                    Ok(users) => users,
                };

                self.merge(on_disk_users);

                let result = serializer.write(self.to_records());

                result
            }
        }
    }

    pub fn clear(&mut self) {
        self.users_to_scores.clear()
    }

    fn merge(&mut self, others: Vec<UserRecord>) {
        for record in others {
            match self.get(&record.id) {
                Some(score) => self.put(record.id, record.score + score, u64::MAX),
                None => self.put(record.id, record.score, u64::MAX),
            };
        }
    }

    fn get(&self, id: &u64) -> Option<&u64> {
        self.users_to_scores.get(id)
    }
}

mod test {
    use super::*;

    #[test]
    fn to_records() {
        let mut users = Users::new();

        users.put(1, 2, 1);
        users.put(3, 4, 1);
        users.put(5, 6, 1);

        let records = users.to_records();
        assert!(records.contains(&UserRecord { id: 1, score: 2 }));
        assert!(records.contains(&UserRecord { id: 3, score: 4 }));
        assert!(records.contains(&UserRecord { id: 5, score: 6 }));
    }
}
