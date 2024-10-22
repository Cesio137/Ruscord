use crate::{Context, Error};
use poise::CreateReply;

#[poise::command(slash_command, prefix_command)]
///Replies with pong 🏓
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.send(CreateReply {
        content: Some("Pong 🏓".to_owned()),
        ..Default::default()
    }).await?;
    Ok(())
}