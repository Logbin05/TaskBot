use dotenvy::dotenv;
use serenity::prelude::*;
use std::env;

pub mod backup;
pub mod cache;
pub mod config;
pub mod handler;

use crate::config::db::create_db_pool;
use crate::handler::connect::Handler;

#[tokio::main]
async fn main() {
    dotenv().ok();
    create_db_pool().await;
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
