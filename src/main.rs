mod datums;
mod errors;
mod processors;
mod serializers;

use crate::serializers::loot_serializer::LootSerializer;
use crate::serializers::serializer::Serializer;
use processors::handler::Handler;
use serenity::client::Client;
use std::fs;
use std::path::Path;

fn main() {
    let token = fs::read_to_string("bot.txt")
        .expect("Please place a bot.txt, containing only your bot token, next to the executable");

    fs::create_dir_all(Path::new(processors::handler::Handler::SCORES_DIRECTORY))
        .expect("Couldn't create storage directory");

    let loot_serializer = LootSerializer::new(Path::new("loot.txt")).expect(
        "Please place a loot.txt file next to the executable. The first row should just say 'item', and all following rows should be the individual items.",
    );
    let mut client =
        Client::new(
            token,
            Handler::new(loot_serializer.read().expect(
                "Could not read items. Please make sure your loot file is properly formatted",
            )),
        )
        .expect("Error creating client");

    println!("Server starting");
    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}
