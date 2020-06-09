use crate::datums::user_record::UserRecord;
use crate::errors::result::Result;
use crate::serializers::serializer::Serializer;
use crate::serializers::user_serializer::UserSerializer;
use std::collections::HashMap;
use std::path::Path;

type ChannelId = u64;
type UserId = u64;

/// A channel id and a user id
type UniqueId = (ChannelId, UserId);

pub struct DataManager {
    queued_users_by_channel: HashMap<UniqueId, u64>,
}

impl DataManager {
    pub fn new() -> Self {
        DataManager {
            queued_users_by_channel: HashMap::new(),
        }
    }

    pub fn put(&mut self, unique_id: UniqueId, score: u64, threshold: u64) -> Option<u64> {
        let result = self.get(&unique_id);
        let previous = match result {
            Some(s) => self.queued_users_by_channel.insert(unique_id, score + s),
            None => self.queued_users_by_channel.insert(unique_id, score),
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

    pub fn to_records(&self, channel_id: ChannelId) -> Vec<UserRecord> {
        let mut results = Vec::new();
        for (unique_id, score) in &self.queued_users_by_channel {
            if unique_id.0 == channel_id {
                results.push(UserRecord {
                    id: unique_id.1,
                    score: *score,
                })
            }
        }

        results
    }

    pub fn size(&self) -> usize {
        self.queued_users_by_channel.len()
    }

    pub fn write(&mut self, path: &Path, channel_id: ChannelId) -> Result<()> {
        match UserSerializer::new(path) {
            Err(e) => Err(e),
            Ok(serializer) => {
                let on_disk_users = match serializer.read() {
                    // Users file doesn't exist yet
                    Err(_) => vec![],
                    Ok(users) => users,
                };

                self.merge(on_disk_users, channel_id.clone());

                let result = serializer.write(self.to_records(channel_id));

                self.clear(channel_id);

                result
            }
        }
    }

    pub fn clear(&mut self, channel_id: ChannelId) {
        let mut ids_to_remove = Vec::new();
        for (unique_id, _) in &self.queued_users_by_channel {
            if unique_id.0 == channel_id {
                ids_to_remove.push(unique_id);
            }
        }
    }

    fn merge(&mut self, others: Vec<UserRecord>, channel_id: ChannelId) {
        for record in others {
            let unique_id = (channel_id.clone(), record.id);
            match self.get(&unique_id) {
                Some(score) => self.put(unique_id, record.score + score, u64::MAX),
                None => self.put(unique_id, record.score, u64::MAX),
            };
        }
    }

    fn get(&self, unique_id: &UniqueId) -> Option<&u64> {
        self.queued_users_by_channel.get(unique_id)
    }
}
