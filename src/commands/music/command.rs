use std::sync::Arc;

use lavalink_rs::prelude::{TrackInQueue, TrackLoadData, SearchEngines};
use poise::ChoiceParameter;
use serenity::futures::StreamExt;
use serenity::futures::{Stream, stream};

use crate::error::Error;
use crate::types::Context;
use crate::commands::music::utils;
use crate::commands::music::types::EngineChoice;


/// autocomplete for title
async fn autocomplete_title<'a>(ctx: Context<'_>, partial: &'a str) -> impl Stream<Item = String> + 'a {
    println!("Partioal data: {}", partial);

    let guild_id = ctx.guild_id().unwrap();
    let ll_client = Arc::clone(&ctx.data().ll_client);
    let query = SearchEngines::YouTube.to_query(partial).unwrap();
    let loaded_tracks = ll_client.load_tracks(guild_id, &query).await.unwrap();

    let tracks: Vec<TrackInQueue> = match loaded_tracks.data {
        Some(TrackLoadData::Track(x)) => vec![x.into()],
        Some(TrackLoadData::Search(x)) => x.iter().map(|t| t.into()).collect(),
        Some(TrackLoadData::Playlist(x)) => x.tracks.iter().map(|x| x.clone().into()).collect(),
        Some(TrackLoadData::Error(err)) => {
            log::error!("Some error in autocomplete_title{}", err.message);
            vec![]
        },
        _ => vec![],
    };

    stream::iter(tracks)
        .map(|track| {
            let title = if track.track.info.title.len() > 80usize {
                track.track.info.title[0..79].to_string()
            } else {
                track.track.info.title
            };

            format!("{}|{}", title, track.track.info.identifier)
        })
}

/// autocomplete for engine
// async fn autocomplete_engine<'a>(_ctx: Context<'_>, partial: &'a str) -> impl Stream<Item = String> + 'a {
//     let choices: Vec<String> = EngineChoice::list().iter().map(|choice| choice.name.clone()).collect();

//     println!("on autocomplete_engine {}", partial);

//     stream::iter(choices)
//         .map(|title| title)
// }

/// Play select audio
#[poise::command(slash_command)]
pub async fn play(
    ctx: Context<'_>,
    #[description = "Select search engine"]
    engine: EngineChoice,
    #[description = "Enter video title / id / link"]
    #[autocomplete = "autocomplete_title"]
    title: String,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();
    let is_joined = utils::join(&ctx, guild_id, None).await?;
    let ll_client = Arc::clone(&ctx.data().ll_client);

    let Some(player) = ll_client.get_player_context(guild_id) else {
        ctx.say("Join the bot to a voice channel first.").await?;
        return Ok(());
    };

    let query = if title.starts_with("http") {
        title
    } else {
        // TODO: add engine options
        SearchEngines::YouTube.to_query(&title)?
    };

    let loaded_tracks = ll_client.load_tracks(guild_id, &query).await?;

    let mut playlist_info = None;

    let tracks: Vec<TrackInQueue> = match loaded_tracks.data {
        Some(TrackLoadData::Track(x)) => vec![x.into()],
        Some(TrackLoadData::Search(x)) => vec![x[0].clone().into()],
        Some(TrackLoadData::Playlist(x)) => {
            playlist_info = Some(x.info);
            x.tracks.iter().map(|x| x.clone().into()).collect()
        }

        _ => {
            ctx.say(format!("{:?}", loaded_tracks)).await?;
            return Ok(());
        }
    };

    if let Some(info) = playlist_info {
        ctx.say(format!("Added playlist to queue: {}", info.name,)).await?;
    } else {
        let track = &tracks[0].track;

        if let Some(uri) = &track.info.uri {
            ctx.say(format!(
                "Added to queue: [{} - {}](<{}>)",
                track.info.author, track.info.title, uri
            ))
            .await?;
        } else {
            ctx.say(format!(
                "Added to queue: {} - {}",
                track.info.author, track.info.title
            ))
            .await?;
        }
    }

    let mut queue = player.get_queue().await?;
    tracks.iter().for_each(|track| queue.push_back(track.clone()));

    player.play(&tracks[0].track).await?;

    if is_joined {
        return Ok(());
    }

    Ok(())
}


/// Leave the current voice channel.
#[poise::command(slash_command, prefix_command)]
pub async fn leave(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();
    let manager = songbird::get(ctx.serenity_context()).await.unwrap().clone();
    let lava_client = Arc::clone(&ctx.data().ll_client);

    lava_client.delete_player(guild_id).await?;

    if manager.get(guild_id).is_some() {
        manager.remove(guild_id).await?;
    }

    ctx.say("Left voice channel.").await?;

    Ok(())
}