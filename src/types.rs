use std::sync::Mutex;

use crate::error::Error;
use crate::services::country::types::Country;

#[derive(Debug, Default)]
pub struct Data {
    pub countrys: Mutex<Option<Vec<Country>>>
}

pub type Context<'a> = poise::Context<'a, Data, Error>;

impl Data {
    pub fn set_countrys(&self, countrys: Vec<Country>) {
        let _ = self.countrys.lock().unwrap().insert(countrys);
    }
}