use serenity::all::{
    Context,
    EventHandler as SerenityEventHandler,
    Message,
};
use serenity::async_trait;
use songbird::events::{
    Event,
    EventContext,
    EventHandler as VoiceEventHandler,
};
use tracing::error;

pub struct Handler;
pub struct TrackErrorNotifier;

#[async_trait]
impl SerenityEventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!verdades" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Daniel é muito guei!").await {
                error!("Error sending message: {:?}", why);
            }
        }
    }
}

#[async_trait]
impl VoiceEventHandler for TrackErrorNotifier {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        if let EventContext::Track(track_list) = ctx {
            for (state, handle) in *track_list {
                error!("Track {:?} encountered an error: {:?}",
                    handle.uuid(),
                    state.playing
                )
            }
        }

        None
    }
}