use serde::{Deserialize, Serialize};

use crate::error::Result;

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all(serialize = "SCREAMING_SNAKE_CASE"))]
pub struct Config {
    pub token: String,
    pub country_url: String,
    pub ow_key: String,
    pub ll_host: String,
    pub ll_pass: String,
}

impl Config {
    pub fn new() -> Result<Config> {
        Ok(envy::from_env::<Config>()?)
    }
}