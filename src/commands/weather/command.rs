use poise::serenity_prelude as serenity;
use ::serenity::futures::StreamExt;
use serenity::futures::{Stream, future, stream};

use crate::error::Error;
use crate::types::Context;

async fn autocomplete_city<'a>(_ctx: Context<'_>, partial: &'a str) -> impl Stream<Item = String> + 'a {
    stream::iter(&["Amanda", "Bob", "Christian", "Danny", "Ester", "Falk"])
        .filter(move |name| future::ready(name.starts_with(partial)))
        .map(|name| name.to_string())
}

/// Displays your or another user's account creation date
#[poise::command(slash_command)]
pub async fn weather(
    ctx: Context<'_>,
    #[description = "Enter city"]
    #[autocomplete = "autocomplete_city"]
    city: Option<String>,
) -> Result<(), Error> {
    let c = city.unwrap_or_else(|| "Uganda".to_string());
    let response = format!("Select {} city", c);

    ctx.say(response).await?;
    Ok(())
}