use poise::serenity_prelude as serenity;
use serenity::{model::id::ChannelId, Http, Mentionable};

use crate::error::Result;
use crate::types::Context;

pub async fn join(
    ctx: &Context<'_>,
    guild_id: serenity::GuildId,
    channel_id: Option<serenity::ChannelId>,
) -> Result<bool> {
    Ok(true)
}