pub mod events;
use serenity::prelude::*;
use crate::commands::{Data, FSlashCommands, HttpClient, HttpKey};
use crate::discord::events::Handler;
use songbird::SerenityInit;

pub struct FApp {
    pub client: Client,
}

impl FApp {
    pub async fn bootstrap(token: &String, intents: GatewayIntents) -> Self {
        let slash_commands = FSlashCommands::loadcommands().await;
        let framework = poise::Framework::builder()
            .options(poise::FrameworkOptions {
                commands: slash_commands.commands,
                ..Default::default()
            })
            .setup(|ctx, _ready, framework| {
                Box::pin(async move {
                    println!("Refreshing commands!");
                    // Delete old commands
                    let result = ctx.http.get_global_commands().await;
                    if let Ok(commands) = result {
                        for command in commands.iter() {
                            ctx.http.delete_global_command(command.id).await.unwrap();
                        }
                    }
                    // Registering/Updating new commands
                    for command in framework.options().commands.iter() {
                        println!("(/) {} command loaded!", command.name)
                    }
                    let result = poise::builtins::register_globally(ctx, &framework.options().commands).await;
                    if let Err(why) = result {
                        println!("Error registering global commands: {:?}", why);
                    }
                    println!("➝ Online as {}", _ready.user.name);
                    Ok(Data {})
                })
            })
            .build();

        let mut client = Client::builder(token, intents)
            .framework(framework)
            .register_songbird()
            .event_handler(Handler)
            .type_map_insert::<HttpKey>(HttpClient::new())
            .await.expect("Error trying to create client");


        if let Err(error) = client.start().await {
            println!("Client error: {error:?}");
        }

        return Self { client };
    }
}