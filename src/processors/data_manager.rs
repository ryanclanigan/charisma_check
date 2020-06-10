use crate::datums::user_record::UserRecord;
use crate::errors::result::Result;
use crate::serializers::serializer::Serializer;
use crate::serializers::user_serializer::UserSerializer;
use std::collections::HashMap;
use std::path::Path;

type ChannelId = u64;
type Score = u64;
type UserId = u64;
type UserToScore = HashMap<UserId, Score>;

pub struct DataManager {
    queued_users_by_channel: HashMap<ChannelId, UserToScore>,
}

impl DataManager {
    pub fn new() -> Self {
        DataManager {
            queued_users_by_channel: HashMap::new(),
        }
    }

    /// Inserts a new user-score combo into the DataManager. If the score is > threshold, return the score,
    /// otherwise returning None
    pub fn put(
        &mut self,
        channel_id: ChannelId,
        new_record: UserRecord,
        threshold: u64,
    ) -> Option<u64> {
        let result = self.get(&channel_id);
        let previous = match result {
            Some(channel) => match channel.get(&new_record.id) {
                Some(score) => channel.insert(new_record.id, score + new_record.score),
                None => channel.insert(new_record.id, new_record.score),
            },
            None => {
                self.queued_users_by_channel
                    .insert(channel_id, HashMap::new());
                self.get(&channel_id)
                    .unwrap()
                    .insert(new_record.id, new_record.score)
            }
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

    pub fn to_records(&self, channel_id: &ChannelId) -> Vec<UserRecord> {
        let mut results = Vec::new();
        match self.queued_users_by_channel.get(channel_id) {
            None => (),
            Some(channel) => {
                for (user, score) in channel {
                    results.push(UserRecord::new(user.clone(), score.clone()))
                }
            }
        }

        results
    }

    pub fn size(&self) -> usize {
        self.queued_users_by_channel.len()
    }

    pub fn size_of_channel(&self, channel_id: &ChannelId) -> usize {
        match self.queued_users_by_channel.get(channel_id) {
            None => 0,
            Some(channel) => channel.len(),
        }
    }

    pub fn write(
        &mut self,
        path: &Path,
        channel_id: &ChannelId,
        winning_score: u64,
    ) -> Result<Vec<UserRecord>> {
        match UserSerializer::new(path) {
            Err(e) => Err(e),
            Ok(serializer) => {
                let on_disk_users = match serializer.read() {
                    // Users file doesn't exist yet
                    Err(_) => vec![],
                    Ok(users) => users,
                };

                self.merge(on_disk_users, channel_id.clone());

                let records_to_write = Vec::new();
                let winners = Vec::new();
                for record in self.to_records(channel_id) {
                    if record.score >= winning_score {
                        records_to_write.push(UserRecord::new(record.id, 0));
                        winners.push(record)
                    } else {
                        records_to_write.push(record)
                    }
                }

                let result = serializer.write(&records_to_write);

                self.clear(channel_id);

                Ok(winners)
            }
        }
    }

    pub fn clear(&mut self, channel_id: &ChannelId) {
        match self.get(channel_id) {
            None => (),
            Some(channel) => channel.clear(),
        }
    }

    fn merge(&mut self, others: Vec<UserRecord>, channel_id: ChannelId) {
        for other in others {
            self.put(channel_id, other, u64::MAX);
        }
    }

    fn get(&mut self, channel_id: &ChannelId) -> Option<&mut UserToScore> {
        self.queued_users_by_channel.get_mut(channel_id)
    }
}
