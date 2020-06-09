mod datums;
mod errors;
mod processors;
mod serializers;

use datums::user_record::UserRecord;
use serenity::client::Client;
use serenity::model::{channel::Message, gateway::Ready};
use serenity::prelude::*;
use serializers::serializer::Serializer;
use serializers::user_serializer::UserSerializer;
use std::fs;
use std::path::Path;

struct Handler;

impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg: Message) {
        // if msg.channel_id == get_followed_channel() {}
        if msg.content == "!jeff" {
            let path = Path::new("f.csv");
            let serializer = UserSerializer::new(&path).unwrap();
            match serializer.write(vec![UserRecord { id: 1, score: 1 }]) {
                Ok(_) => (),
                Err(e) => println!("{}", e.to_string()),
            };
        }
    }

    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

static mut FOLLOWED_CHANNEL: Option<u64> = None;

fn get_followed_channel() -> u64 {
    unsafe {
        return match FOLLOWED_CHANNEL {
            // TODO: Figure out way to choose followed channel. DB maybe?
            None => panic!("No followed channel set."),
            Some(c) => c,
        };
    }
}

fn main() {
    let token = fs::read_to_string("bot.txt")
        .expect("Please place a bot.txt, containing only your bot token, next to the executable");
    let mut client = Client::new(token, Handler).expect("Error creating client");

    println!("Server starting");
    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}
