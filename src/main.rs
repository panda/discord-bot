use std::env;

use tokio::prelude::*;

use serenity::{
    async_trait,
    model::{channel::Message},
    prelude::*,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!boba" {
            if let Err(msg) = msg.channel_id.say(&ctx.http, "tea!").await {
                println!("There was an error sending message: {:?}", msg);
            }
        }
    }
}

#[tokio::main]
async fn main() {

    let env = kankyo::load(true); // Needed kankyo to load my .environments file in root directory

    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");

    let mut client = Client::new(&token)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(msg) = client.start().await {
        println!("Err: {:?}", msg);
    }
}