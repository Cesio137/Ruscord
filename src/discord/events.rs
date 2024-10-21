use serenity::all::{
    Context, 
    EventHandler, 
    Message
};
use serenity::async_trait;
use songbird::events::{
    Event, 
    EventContext, 
    EventHandler as VoiceEventHandler
};

pub struct Handler;
pub struct TrackErrorNotifier;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!verdades" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Daniel é muito guei!").await {
                println!("Error sending message: {why:?}");
            }
        }
    }
}

#[async_trait]
impl VoiceEventHandler for TrackErrorNotifier {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        if let EventContext::Track(track_list) = ctx {
            for (state, handle) in *track_list {
                println!(
                    "Track {:?} encountered an error: {:?}",
                    handle.uuid(),
                    state.playing
                );
            }
        }

        None
    }
}