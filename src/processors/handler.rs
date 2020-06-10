use super::data_manager::DataManager;
use crate::datums::user_record::UserRecord;
use crate::processors::scorer::Scorer;
use serenity::model::{channel::Message, gateway::Ready};
use serenity::prelude::*;
use std::path::Path;
use std::sync::{Arc, Mutex};

pub struct Handler {
    data_manager: Arc<Mutex<DataManager>>,
}

impl Handler {
    pub fn new() -> Self {
        Handler {
            data_manager: Arc::new(Mutex::new(DataManager::new())),
        }
    }

    const MAX_USER_COUNT_BEFORE_WRITE: usize = 4;
    const SCORE_MODULO_BEFORE_WRITE: u64 = 10;
    const WINNING_SCORE: u64 = 53;
    pub const SCORES_DIRECTORY: &'static str = "scores";

    /// Updates the user scores with the current message context, and then writes if
    /// the score thresholds have been met.
    fn update_and_maybe_write_users(&self, message: &Message) {
        let channel_id = *message.channel_id.as_u64();
        let mut data_manager = self.data_manager.lock().unwrap();
        let result = data_manager.put(
            channel_id,
            UserRecord::new(
                *message.author.id.as_u64(),
                Scorer::score_message(message.content.clone()),
            ),
            Handler::SCORE_MODULO_BEFORE_WRITE,
        );

        if data_manager.size() == Handler::MAX_USER_COUNT_BEFORE_WRITE || result.is_some() {
            match data_manager.write(
                Path::new(&format!(
                    "{}/{}{}",
                    Handler::SCORES_DIRECTORY,
                    message.channel_id.0.to_string(),
                    ".csv"
                )),
                &channel_id,
                Handler::WINNING_SCORE,
            ) {
                Err(e) => println!("{}", e.to_string()),
                Ok(winners) => {
                    println!("Wrote scores for channel {}", channel_id);
                    if winners.len() > 0 {
                        
                    }
                }
            }
        }
    }
}

impl EventHandler for Handler {
    fn message(&self, context: Context, message: Message) {
        // if msg.channel_id == get_followed_channel() {}
        let me = {
            let cache = &context.cache.read();
            cache.user.id
        };
        match message.channel(&context) {
            None => panic!("Channel not found in cache. Something bad has happened."),
            Some(channel) => {
                match channel.guild() {
                    Some(guild) => {
                        let read_lock = guild.read();
                        let guild_name = read_lock.name();
                        if guild_name.to_lowercase().contains("roleplay") && me != message.author.id
                        {
                            self.update_and_maybe_write_users(&message);
                        }
                    }
                    None => panic!(
                        "Lootbot linked to a channel that is not a guild channel! Channel Id: {}",
                        message.channel_id
                    ),
                };
            }
        }
    }

    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}
