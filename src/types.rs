use std::sync::Mutex;

use crate::error::Error;
use crate::services::country::types::{Country, City};

#[derive(Debug, Default)]
pub struct Data {
    pub countrys: Mutex<Option<Vec<Country>>>,
    pub cities: Mutex<Option<Vec<City>>>
}

pub type Context<'a> = poise::Context<'a, Data, Error>;

impl Data {
    pub fn set_cities(&self, cities: Vec<City>) -> Vec<City> {
        let _ = self.cities.lock().unwrap().insert(cities.clone());
        cities
    }
}