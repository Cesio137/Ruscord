mod discord;
mod settings;
mod commands;
use serenity::prelude::*;

#[tokio::main]
async fn main() {
    let envschema = settings::FEnvConfig::validate_schema();    
    let _ = discord::FApp::bootstrap(&envschema.bot_token, GatewayIntents::non_privileged()).await; 
}

