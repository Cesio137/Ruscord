use crate::{Context, Error};
use poise::serenity_prelude as serenity;

#[poise::command(slash_command, prefix_command)]
///Displays your or another user's account creation date.
pub async fn age(ctx: Context<'_>, #[description = "Selected user"] user: Option<serenity::User>) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at().format("%d/%m/%Y %H:%M"));
    ctx.say(response).await?;
    Ok(())
}