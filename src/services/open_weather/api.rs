use std::sync::Arc;

use reqwest;

use crate::{config::Config, services::open_weather::types::OpenWeather};

impl OpenWeather {
    pub async fn by_city(city: &str, cfg: Arc<Config>) -> Self {
        let url = format!("https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric", city, &cfg.ow_key);

        reqwest::get(url)
            .await.unwrap()
            .json::<Self>()
            .await.unwrap()
    }
}