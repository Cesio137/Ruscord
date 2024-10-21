pub mod ping;
pub mod player;
pub mod age;
use crate::{Data, Error};
use poise::Command;
pub use {
    age::age,
    ping::ping,
    player::player,
};

pub fn load_commands() -> Vec<Command<Data, Error>> {
    vec![
        ping(),
        player(),
        age()
    ]
}