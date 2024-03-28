use std::sync::Arc;

use poise::serenity_prelude as serenity;
use serenity::futures::StreamExt;
use serenity::futures::{Stream, future, stream};
use serenity::model::Colour;

use crate::error::Error;
use crate::types::{Context, Data};
use crate::services::country::types::City;
use crate::services::open_weather::types::OpenWeather;

/// autocomplete for city input choice
async fn autocomplete_city<'a>(ctx: Context<'_>, partial: &'a str) -> impl Stream<Item = String> + 'a {
    let app_data = ctx.data();
    let cities = app_data.cities.lock().unwrap().clone();

    let cities: Vec<City> = match cities {
        Some(data) => data,
        None => app_data.set_cities(City::all(Arc::clone(&app_data.config)).await.unwrap()),
    };

    stream::iter(cities)
        .filter(move |country| future::ready(country.title.to_lowercase().starts_with(partial)))
        .map(|country| country.title)
}

/// Displays current weather on select country -> city
#[poise::command(slash_command)]
pub async fn weather(
    ctx: Context<'_>,
    #[description = "Enter city"]
    #[autocomplete = "autocomplete_city"]
    city: String,
) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;

    let Data { config, .. } = ctx.data();
    let weather_data = OpenWeather::by_city(&city, Arc::clone(&config)).await;

    let temp_field = format!(
        "```fix\nCurrent ➜ {} °C\nMin ➜ {}\nMax ➜ {}\n```",
        weather_data.main.feels_like,
        weather_data.main.temp_min,
        weather_data.main.temp_max,
    ); 

    let wind_field = format!(
        "```fix\nSpeed ➜ {} M/s\nDeg ➜ {}\n```",
        weather_data.wind.speed,
        weather_data.wind.deg,
    );

    let misc_field = format!(
        "```fix\nSunrise ➜ {} Unix\nSunset ➜ {} Unix\n```",
        weather_data.sys.sunrise,
        weather_data.sys.sunset,
    );

    let embed = serenity::CreateEmbed::new()
        .description(format!("Weather in {}", weather_data.name))
        .field("Temp 🌡️", temp_field, false)
        .field("Wind 💨", wind_field, false)
        .field("Misc 🗃️", misc_field, false)
        .color(Colour::BLUE);
    
    let builder = serenity::builder::CreateMessage::new()
        .add_embed(embed);

    ctx.channel_id().send_message(ctx.http(), builder).await?;

    Ok(())
}