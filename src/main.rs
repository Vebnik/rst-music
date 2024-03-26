mod config;
mod error;
mod commands;
mod types;

use config::Config;
use types::Data;

// poise
use poise::serenity_prelude as serenity;


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
    let cmds = vec![
        commands::test::command::age(),
    ];

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {commands: cmds, ..Default::default()})
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let mut client = serenity::ClientBuilder::new(&cfg.token, intents)
        .framework(framework)
        .await
        .unwrap();

    client.start().await.unwrap();

    Ok(())
}
