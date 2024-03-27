mod config;
mod error;
mod commands;
mod types;
mod services;

use config::Config;
use types::Data;

use poise::serenity_prelude as serenity;
use std::sync::Mutex;

#[tokio::main]
async fn main() -> Result<(), ()> {
    // Init env
    dotenv::dotenv().ok();

    // Init logger
    env_logger::init();

    // Create config
    let cfg = Config::new().unwrap();

    // bot config ...
    let intents = serenity::GatewayIntents::non_privileged();

    // commands scope
    let cmds = commands::get_cmds();
    println!("Try to register {} commands", cmds.capacity());

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {commands: cmds, ..Default::default()})
        .setup(|ctx, ready, framework| {
            println!("Setup as {} app", ready.user.name);

            ctx.idle();

            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {countrys: Mutex::new(None)})
            })
        })
        .build();

    let mut client = serenity::ClientBuilder::new(&cfg.token, intents)
        .framework(framework)
        .await
        .unwrap();
    
    println!("Started app ...");
    client.start().await.unwrap();

    Ok(())
}
