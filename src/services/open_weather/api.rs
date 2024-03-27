use reqwest;

use crate::services::open_weather::types::OpenWeather;

impl OpenWeather {
    pub async fn by_city(city: &str) -> Self {
        let url = format!("https://api.openweathermap.org/data/2.5/weather?q={}&appid=a70ff5078adfb84d5d0958bb1e13483c", city);

        reqwest::get(url)
            .await.unwrap()
            .json::<Self>()
            .await.unwrap()
    }
}