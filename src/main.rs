use serenity::prelude::*;
use std::env;
use dotenvy::dotenv;

pub mod backup;
pub mod cache;
pub mod config;
pub mod handler;

use crate::handler::connect::Handler;

#[tokio::main]
async fn main() {
  dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("Error a token in the environment");
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
