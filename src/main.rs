mod datums;
mod errors;
mod processors;
mod serializers;

use processors::handler::Handler;
use serenity::client::Client;
use std::fs;
use std::path::Path;

fn main() {
    let token = fs::read_to_string("bot.txt")
        .expect("Please place a bot.txt, containing only your bot token, next to the executable");

    fs::create_dir_all(Path::new(processors::handler::Handler::SCORES_DIRECTORY))
        .expect("Couldn't create storage directory");

    let mut client = Client::new(token, Handler::new()).expect("Error creating client");

    println!("Server starting");
    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}
