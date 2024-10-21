use std::env;
use dotenv::dotenv;

pub struct FEnvConfig {
    pub bot_token: String,
    pub webhook_logs_url: String
}

impl FEnvConfig {
    pub fn validate_schema() -> Self {
        dotenv().expect("❌ Error trying to load .env file.");
        let token = env::var("BOT_TOKEN").expect("❌ Bot token is missing.");
        let webhook = env::var("WEBHOOK_LOGS_URL").unwrap();
        println!("✔ Env vars loaded successfully!");
        return Self {
            bot_token: token,
            webhook_logs_url: webhook
        };
    }
}