use serenity::{async_trait, model::channel::Message, prelude::*};
pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
  async fn message(&self, ctx: Context, msg: Message) {
    if msg.content.starts_with('?') {
      match msg.content.as_str() {
          "?ping" => {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
              println!("Error sending message: {why:?}")
            }
          }
          _ => {
            println!("Upp... this command unknown")
          }
      }
    }
  }
}