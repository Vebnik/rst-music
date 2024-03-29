mod config;
mod error;
mod commands;
mod types;
mod services;

use config::Config;
use songbird::SerenityInit;
use types::Data;
use services::lavalink::handlers;

use poise::serenity_prelude as serenity;
use lavalink_rs::prelude as lavalink;
use lavalink_rs::model::events;
use std::sync::{Mutex, Arc};


#[tokio::main]
async fn main() -> Result<(), ()> {
    // Init env
    dotenv::dotenv().ok();

    // Init logger
    env_logger::init();

    // Create config
    let cfg = Config::new().unwrap();

    // lavalink config
    let events = events::Events {
        raw: Some(handlers::raw_event),
        ready: Some(handlers::ready_event),
        track_start: Some(handlers::track_start),
        ..Default::default()
    };

    let node = lavalink::NodeBuilder {
        hostname: cfg.ll_host.clone(),
        password: cfg.ll_pass.clone(),
        is_ssl: false,
        session_id: None,
        user_id: 918436440633925653.into(),
        events: events::Events::default(),
    };

    let ll_client = lavalink::LavalinkClient::new(
        events,
        vec![node],
        lavalink::NodeDistributionStrategy::round_robin(),
    ).await;

    // global state
    let data = Data {
        countrys: Arc::new(Mutex::new(None)),
        cities: Arc::new(Mutex::new(None)),
        config: Arc::new(cfg.clone()),
        ll_client: Arc::new(ll_client.clone()),
    };

    // commands scope
    let cmds = commands::get_cmds();
    println!("Try to register {} commands", cmds.capacity());

    // poise framework init
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {commands: cmds, ..Default::default()})
        .setup(|ctx, ready, framework| {
            println!("Setup as {} app", ready.user.name);

            ctx.idle();

            Box::pin(async move {
                for guild in ready.guilds.iter() {
                    poise::builtins::register_in_guild(ctx, &framework.options().commands, guild.id).await?;
                };
                Ok(data)
            })
        })
        .build();
    
    // create client
    let mut client = serenity::ClientBuilder::new(&cfg.token, serenity::GatewayIntents::all())
        .register_songbird()
        .framework(framework)
        .await
        .unwrap();
    
    println!("Started app ...");
    client.start().await.unwrap();

    Ok(())
}
