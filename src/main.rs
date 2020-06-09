mod datums;
mod errors;
mod processors;
mod serializers;

use processors::handler::Handler;
use serenity::client::Client;
use std::fs;

fn main() {
    let token = fs::read_to_string("bot.txt")
        .expect("Please place a bot.txt, containing only your bot token, next to the executable");
    // let followed_channel = fs::read_to_string("channel.txt").expect(
    //     "Please place a channel.txt, containing only your channel id, next to the executable",
    // );

    let mut client = Client::new(token, Handler::new()).expect("Error creating client");

    println!("Server starting");
    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}
