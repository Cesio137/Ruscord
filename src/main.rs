mod env;
mod cmd;
mod events;

use crate::cmd::load_commands;
use events::Handler;
use reqwest::Client as HttpClient;
use serenity::prelude::*;
use songbird::SerenityInit;
use tracing::{error, info, warn};

pub struct Data {}
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
pub struct HttpKey;
impl TypeMapKey for HttpKey {
    type Value = HttpClient;
}

fn get_intents() -> GatewayIntents {
    GatewayIntents::all()
}

async fn build_framework() -> poise::Framework<Data, Error> {
    poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: load_commands(),
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                info!("Refreshing cmd!");
                // Delete old cmd
                let result = ctx.http.get_global_commands().await;
                if let Ok(commands) = result {
                    for command in commands.iter() {
                        ctx.http.delete_global_command(command.id).await.unwrap();
                    }
                }
                // Registering/Updating new cmd
                for command in framework.options().commands.iter() {
                    info!("(/) {} command loaded!", command.name);
                }
                let result = poise::builtins::register_globally(ctx, &framework.options().commands).await;
                if let Err(why) = result {
                    warn!("Error registering global cmd: {:?}", why)
                }
                info!("➝ Online as {}", _ready.user.name);
                Ok(Data {})
            })
        })
        .build()
}

async fn bootstrap(token: &String, intents: GatewayIntents) -> Client {
    let mut client = Client::builder(token, intents)
        .framework(build_framework().await)
        .register_songbird()
        .event_handler(Handler)
        .type_map_insert::<HttpKey>(HttpClient::new())
        .await.expect("Error trying to create client");


    if let Err(error) = client.start().await {
        error!("Client error: {error:?}");
    }

    return client;
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .compact()
        .with_file(true)
        .with_line_number(true)
        .init();
    let envschema = env::validate_schema();
    let _ = bootstrap(&envschema.bot_token, get_intents()).await;
}

