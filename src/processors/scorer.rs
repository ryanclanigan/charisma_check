use serenity::model::channel::Message;

pub struct Scorer;

impl Scorer {
    pub fn score_message(message: String) -> u64 {
        1
    }
}
