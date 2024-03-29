use std::sync::{Mutex, Arc};
use lavalink_rs::client::LavalinkClient;
use poise::serenity_prelude as serenity;
use serenity::{model::id::ChannelId, Http};

use crate::error::Error;
use crate::services::country::types::{Country, City};
use crate::config::Config;

#[derive(Debug)]
pub struct Data {
    pub countrys: Arc<Mutex<Option<Vec<Country>>>>,
    pub cities: Arc<Mutex<Option<Vec<City>>>>,
    pub config: Arc<Config>,
    pub ll_client: Arc<LavalinkClient>,
}

pub type Context<'a> = poise::Context<'a, Data, Error>;

pub type PlayerContextData = (ChannelId, Arc<Http>);

impl Data {
    pub fn set_cities(&self, cities: Vec<City>) -> Vec<City> {
        let _ = self.cities.lock().unwrap().insert(cities.clone());
        cities
    }
}