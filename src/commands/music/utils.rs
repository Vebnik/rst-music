use std::sync::Arc;
use std::ops::Deref;
use poise::serenity_prelude as serenity;
use serenity::Mentionable;

use crate::error::Result;
use crate::types::{Context, PlayerContextData};

pub async fn join(
    ctx: &Context<'_>,
    guild_id: serenity::GuildId,
    channel_id: Option<serenity::ChannelId>,
) -> Result<bool> {
    let lava_client = Arc::clone(&ctx.data().ll_client);
    let serenity_context = ctx.serenity_context();

    let manager = match songbird::get(serenity_context).await {
        Some(mng) => mng,
        _ => {
            log::error!("Not found serenity_context");
            return Ok(false);
        }
    };

    if lava_client.get_player_context(guild_id).is_none() {
        let connect_to = match channel_id {
            Some(x) => x,
            None => {
                let guild = ctx.guild().unwrap().deref().clone();
                let user_channel_id = guild
                    .voice_states
                    .get(&ctx.author().id)
                    .and_then(|voice_state| voice_state.channel_id);

                match user_channel_id {
                    Some(channel) => channel,
                    None => {
                        ctx.say("Not in a voice channel").await?;
                        return Err("Not in a voice channel".into());
                    }
                }
            }
        };

        let handler = manager.join_gateway(guild_id, connect_to).await;

        match handler {
            Ok((connection_info, _)) => {
                let context_data: PlayerContextData = (ctx.channel_id(), ctx.serenity_context().http.clone());

                lava_client.create_player_context_with_data::<PlayerContextData>(
                    guild_id,
                    connection_info,
                    Arc::new(context_data),
                )
                .await?;

                ctx.say(format!("Joined {}", connect_to.mention())).await?;

                return Ok(true);
            }
            Err(why) => {
                ctx.say(format!("Error joining the channel: {}", why)).await?;

                return Err(why.into());
            }
        }
    }

    Ok(false)
}
