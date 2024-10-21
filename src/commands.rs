pub mod ping;
pub mod player;
pub mod age;
pub use {
    ping::ping,
    player::player,
    age::age
};
use poise::Command;
use serenity::prelude::*;
pub use reqwest::Client as HttpClient;


pub struct Data {} // User data, which is stored and accessible in all command invocations
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
pub struct HttpKey;

impl TypeMapKey for HttpKey {
    type Value = HttpClient;
}

pub struct FSlashCommands {
    pub commands: Vec<Command<Data, Error>>,
}

impl FSlashCommands {
    pub async fn loadcommands() -> Self {
        let command_list = vec![
            ping(),
            player(),
            age()
        ];

        return Self { commands: command_list };
    }
}