use serde::{Deserialize, Serialize};

use crate::error::Result;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all(serialize = "SCREAMING_SNAKE_CASE"))]
pub struct Config {
    pub token: String,
}

impl Config {
    pub fn new() -> Result<Config> {
        Ok(envy::from_env::<Config>()?)
    }
}