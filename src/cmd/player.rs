use crate::events::TrackErrorNotifier;
use crate::{Context, Error, HttpKey};
use poise::CreateReply;
use serenity::all::GuildId;
use songbird::{
    input::YoutubeDl,
    TrackEvent,
};
use url::Url;

#[poise::command(slash_command, subcommands("add", "pause", "resume", "skip", "stop", "queue"))]
///Commands to control your podcasts and OST queue!
pub async fn player(ctx: Context<'_>) -> Result<(), Error> {
    ctx.send(CreateReply {
        content: Some("Use `/player add, pause, resume, skip, stop or queue` to manage songs.".to_owned()),
        ..Default::default()
    }).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
///Add your favorite podcast or OST to queue!
async fn add(ctx: Context<'_>, #[description = "Enter a name or url."] search: String) -> Result<(), Error> {
    let guild_id = match ctx.guild_id() {
        Some(id) => id,
        None => {
            ctx.send(CreateReply {
                content: Some("You are not connected to a guild!".to_owned()),
                ..Default::default()
            }).await?;
            return Ok(());
        }
    };

    if ctx.cache().guild(guild_id).is_none() {
        ctx.send(CreateReply {
            content: Some("Guild not found in cache.".to_owned()),
            ..Default::default()
        }).await?;
        return Ok(());
    }
    let guild = ctx.cache().guild(guild_id).unwrap().clone();

    let voice_states = match guild.voice_states.get(&ctx.author().id) {
        Some(voice_states) => voice_states,
        None => {
            ctx.send(CreateReply {
                content: Some("Voice state not found.".to_owned()),
                ..Default::default()
            }).await?;
            return Ok(());
        }
    };

    let voice = match voice_states.channel_id {
        Some(voice) => voice,
        None => {
            ctx.send(CreateReply {
                content: Some("You are not connected to a voice channel!".to_owned()),
                ..Default::default()
            }).await?;
            return Ok(());
        }
    };

    ctx.defer().await?;

    let player = match songbird::get(ctx.serenity_context()).await {
        Some(player) => player.clone(),
        None => {
            ctx.send(CreateReply {
                content: Some("Can not initialize player!".to_owned()),
                ..Default::default()
            }).await?;
            return Ok(());
        }
    };

    if player.join(guild_id, voice).await.is_err() {
        ctx.send(CreateReply {
            content: Some("You are not connected to a voice channel!".to_owned()),
            ..Default::default()
        }).await?;
        return Ok(());
    }


    let is_url = Url::parse(search.as_str()).is_ok();
    let http_client = {
        let data = ctx.serenity_context().data.read().await;
        let client = match data.get::<HttpKey>() {
            Some(key) => key.clone(),
            None => {
                ctx.send(CreateReply {
                    content: Some("Can not play the sound!".to_owned()),
                    ..Default::default()
                }).await?;
                return Ok(());
            }
        };
        client
    };

    let handler_lock = match player.get(guild_id) {
        Some(handler) => handler,
        None => {
            ctx.send(CreateReply {
                content: Some("Can not play the sound!".to_owned()),
                ..Default::default()
            }).await?;
            return Ok(());
        }
    };

    let mut handler = handler_lock.lock().await;
    handler.add_global_event(TrackEvent::Error.into(), TrackErrorNotifier);
    let src = if is_url {
        YoutubeDl::new(http_client, search)
    } else {
        YoutubeDl::new_search(http_client, search)
    };
    handler.play_input(src.clone().into());

    ctx.send(CreateReply {
        content: Some("Playing!".to_owned()),
        ..Default::default()
    }).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
///Pause the current track!
pub async fn pause(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id: GuildId = match ctx.guild_id() {
        Some(id) => id,
        None => {
            ctx.send(CreateReply {
                content: Some("You are not connected to a guild!".to_owned()),
                ..Default::default()
            }).await?;
            return Ok(());
        }
    };

    let player = match songbird::get(ctx.serenity_context()).await {
        Some(player) => player,
        None => {
            ctx.send(CreateReply {
                content: Some("Can not initialize player!".to_owned()),
                ..Default::default()
            }).await?;
            return Ok(());
        }
    };

    let handler_lock = match player.get(guild_id) {
        Some(handler) => handler,
        None => {
            ctx.send(CreateReply {
                content: Some("You are not in a voice channel".to_owned()),
                ..Default::default()
            }).await?;
            return Ok(());
        },
    };

    ctx.defer().await;

    let mut handler = handler_lock.lock().await;
    match handler.queue().pause() {
        Ok(_) => {}
        Err(_) => {
            ctx.send(CreateReply {
                content: Some("The current track is already paused!".to_owned()),
                ..Default::default()
            }).await?;
            return Ok(());
        }
    }

    ctx.send(CreateReply {
        content: Some("Current track has been paused!".to_owned()),
        ..Default::default()
    }).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
///Resume the current track!
pub async fn resume(ctx: Context<'_>) -> Result<(), Error> {
    ctx.send(CreateReply {
        content: Some("Pong!".to_owned()),
        ..Default::default()
    }).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
///Skip a certain numbers of track!
pub async fn skip(ctx: Context<'_>, #[description = "Amount of tracks to skip."] amount: Option<u8>) -> Result<(), Error> {
    ctx.send(CreateReply {
        content: Some("Pong!".to_owned()),
        ..Default::default()
    }).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
///Stop and clean the queue!
pub async fn stop(ctx: Context<'_>) -> Result<(), Error> {
    ctx.send(CreateReply {
        content: Some("Pong!".to_owned()),
        ..Default::default()
    }).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
///Show the tracks on the queue.
pub async fn queue(ctx: Context<'_>) -> Result<(), Error> {
    ctx.send(CreateReply {
        content: Some("Pong!".to_owned()),
        ..Default::default()
    }).await?;
    Ok(())
}