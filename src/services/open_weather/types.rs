use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub  struct Main {
    pub temp: f32,
    pub feels_like: f32,
    pub temp_min: f32,
    pub temp_max: f32,
    pub pressure: i32,
    pub humidity: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Wind {
    pub speed: f32,
    pub deg: i32
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Sys {
    pub country: String,
    pub sunrise: i64,
    pub sunset: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OpenWeather {
    pub main: Main,
    pub wind: Wind,
    pub name: String,
    pub timezone: i32,
    pub visibility: i32,
    pub sys: Sys
}