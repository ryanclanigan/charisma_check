use super::user_record::UserRecord;
use std::collections::HashMap;

pub struct Users {
    usersToScores: HashMap<u64, u64>,
}

impl Users {
    fn new() -> Self {
        Users {
            usersToScores: HashMap::new(),
        }
    }

    fn put(&mut self, user: u64, score: u64) {
        self.usersToScores.insert(user, score);
    }

    fn to_records(&self) -> Vec<UserRecord> {
        self.usersToScores
            .iter()
            .map(|(id, score)| UserRecord {
                id: *id,
                score: *score,
            })
            .collect()
    }

    fn is_empty(&self) -> bool {
        self.usersToScores.is_empty()
    }

    fn clear(&mut self) {
        self.usersToScores.clear();
    }
}

mod test {
    use super::*;

    #[test]
    fn to_records() {
        let mut users = Users::new();

        users.put(1, 2);
        users.put(3, 4);
        users.put(5, 6);

        let records = users.to_records();
        assert!(records.contains(&UserRecord { id: 1, score: 2 }));
        assert!(records.contains(&UserRecord { id: 3, score: 4 }));
        assert!(records.contains(&UserRecord { id: 5, score: 6 }));
    }
}
