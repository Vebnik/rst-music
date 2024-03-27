use poise::serenity_prelude as serenity;
use serenity::futures::StreamExt;
use serenity::futures::{Stream, future, stream};

use crate::error::Error;
use crate::types::Context;
use crate::services::country::types::Country;

async fn autocomplete_city<'a>(ctx: Context<'_>, partial: &'a str) -> impl Stream<Item = String> + 'a {
    let app_data = ctx.data();
    let opt_country = app_data.countrys.lock().unwrap().clone();

    let countrys = match opt_country {
        Some(data) => data,
        None => {
            let data = Country::all().await.unwrap();
            app_data.set_countrys(data.clone());
            data
        },
    };

    stream::iter(countrys)
        .filter(move |country| future::ready(country.title.starts_with(partial)))
        .map(|country| country.title)
}

/// Displays current weather on select city
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