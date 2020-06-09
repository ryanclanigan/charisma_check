use serenity::model::channel::Message;

pub struct Scorer;

impl Scorer {
    pub fn new() -> Self {
        Scorer {}
    }

    pub fn score_message(message: Message) -> u64 {
        1
    }
}
