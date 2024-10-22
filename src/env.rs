use dotenvy::dotenv;
use std::env;
use tracing::info;

pub struct FEnvConfig {
    pub bot_token: String,
    pub webhook_logs_url: String,
}

pub fn validate_schema() -> FEnvConfig {
    dotenv().expect("❌ Error trying to load .env file.");
    let token = env::var("BOT_TOKEN").expect("❌ Bot token is missing.");
    let webhook = env::var("WEBHOOK_LOGS_URL").unwrap_or_else(|_| "".to_owned());
    info!("✔ Env vars loaded successfully!");
    FEnvConfig {
        bot_token: token,
        webhook_logs_url: webhook,
    }
}