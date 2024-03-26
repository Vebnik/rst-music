mod config;
mod error;

use config::Config;
use error::Result;

use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {why:?}");
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Init env
    dotenv::dotenv().ok();

    // Init logger
    env_logger::init();

    // Create config
    let cfg = Config::new()?;

    // bot config ...
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client, logging in as a bot.
    let mut client = Client::builder(&cfg.token, intents)
        .event_handler(Handler).await?;

    // Start listening
    if let Err(why) = client.start().await {
        log::error!("Client error: {:?}", why);
    }

    Ok(())
}
