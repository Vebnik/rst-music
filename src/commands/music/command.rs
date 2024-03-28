use std::sync::Arc;

use lavalink_rs::model::search::SearchEngines;
use lavalink_rs::model::track::PlaylistInfo;
use lavalink_rs::prelude::{TrackInQueue, TrackLoadData};
use serenity::futures::StreamExt;
use serenity::futures::{Stream, stream};

use crate::error::Error;
use crate::types::Context;
use crate::commands::music::utils;

async fn autocomplete_title<'a>(ctx: Context<'_>, partial: &'a str) -> impl Stream<Item = String> + 'a {
    let guild_id = ctx.guild_id().unwrap();
    let ll_client = Arc::clone(&ctx.data().ll_client);
    let query = SearchEngines::YouTube.to_query(partial).unwrap();
    let loaded_tracks = ll_client.load_tracks(guild_id, &query).await.unwrap();

    let mut playlist_info: Option<PlaylistInfo> = None;

    let tracks: Vec<TrackInQueue> = match loaded_tracks.data {
        Some(TrackLoadData::Track(x)) => vec![x.into()],
        Some(TrackLoadData::Search(x)) => x.iter().map(|t| t.into()).collect(),
        Some(TrackLoadData::Playlist(x)) => {
            playlist_info = Some(x.info);
            x.tracks.iter().map(|x| x.clone().into()).collect()
        },
        Some(TrackLoadData::Error(err)) => {
            log::error!("Some error in autocomplete_title{}", err.message);
            vec![]
        },
        _ => vec![],
    };

    dbg!(&tracks);
    dbg!(&playlist_info);

    stream::iter(tracks).map(|track| track.track.info.title)
}

/// Play select audio
#[poise::command(slash_command)]
pub async fn play(
    ctx: Context<'_>,
    #[description = "Enter video title / id / link"]
    #[autocomplete = "autocomplete_title"]
    title: String,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();
    let is_joined = utils::join(&ctx, guild_id, None).await?;
    let ll_client = Arc::clone(&ctx.data().ll_client);

    let player = match ll_client.get_player_context(guild_id) {
        Some(res) => res,
        None => {
            ctx.say("Join the bot to a voice channel first.").await?;
            return Ok(());
        }
    };

    dbg!(player);

    ctx.say(format!("You pick: {}, has joined: {}", title, is_joined)).await?;

    Ok(())
}