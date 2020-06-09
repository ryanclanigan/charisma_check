use crate::datums::users::Users;
use crate::processors::scorer::Scorer;
use crate::serializers::serializer::Serializer;
use crate::serializers::user_serializer::UserSerializer;
use serenity::model::channel::*;
use serenity::model::{channel::Message, gateway::Ready};
use serenity::prelude::*;
use std::path::Path;
use std::sync::{Arc, Mutex};

pub struct Handler {
    users: Arc<Mutex<Users>>,
}

impl Handler {
    pub fn new() -> Self {
        Handler {
            users: Arc::new(Mutex::new(Users::new())),
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
        match message.channel(&context.cache) {
            None => panic!("Channel not found in cache. Something bad has happened."),
            Some(channel) => {
                match channel.guild() {
                    Some(guild) => {
                        let read_lock = guild.read();
                        let guild_name = read_lock.name();
                        if guild_name.to_lowercase().contains("roleplay") && me != message.author.id
                        {
                            let mut users = self.users.lock().unwrap();
                            let result = users.put(
                                message.author.id.0,
                                Scorer::score_message(message.content.clone()),
                                10,
                            );

                            if users.size() == 4 || result.is_some() {
                                let records = users.to_records();
                                match UserSerializer::new(Path::new(
                                    &(message.channel_id.0.to_string() + ".csv"),
                                )) {
                                    Err(e) => println!(
                                        "Couldn't write records to disk. Error: {}",
                                        e.to_string()
                                    ),
                                    Ok(s) => match s.write(records) {
                                        Err(e) => println!("{}", e),
                                        Ok(_) => match message.reply(context, "Laserbeam") {
                                            Err(e) => println!("{}", e),
                                            _ => (),
                                        },
                                    },
                                };
                            }
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
