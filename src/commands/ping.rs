use poise::CreateReply;
use crate::commands::{Context, Error};

#[poise::command(slash_command, prefix_command)]
///Replies with pong 🏓
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let reply = CreateReply {
        content: Some("Pong 🏓".to_owned()),
        ..Default::default()
    };
    ctx.send(reply).await?;
    Ok(())
}