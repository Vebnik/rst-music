use reqwest;

use super::types::{Country, City};
use crate::error::Result;

impl Country {
    pub async fn all() -> Result<Vec<Self>> {
        log::debug!("Get all Country");

        let data = reqwest::get("https://raw.githubusercontent.com/russ666/all-countries-and-cities-json/master/countries.json")
            .await?
            .json::<serde_json::Value>()
            .await?;

        let keys: Vec<Country> = data
            .as_object().unwrap()
            .keys()
            .map(|key| {
                let cities = data[key]
                    .as_array().unwrap()
                    .iter()
                    .map(|el| City { title: el.as_str().unwrap().to_owned() })
                    .collect();

                Country {title: key.clone(), cities}
            })
            .collect();

        Ok(keys)
    }
}

impl City {
    pub async fn all() -> Result<Vec<Self>> {
        log::debug!("Get all City");

        let mut compare_cities: Vec<Self> = vec![];

        let data = reqwest::get("https://raw.githubusercontent.com/russ666/all-countries-and-cities-json/master/countries.json")
            .await?
            .json::<serde_json::Value>()
            .await?;

        for key in data.as_object().unwrap().keys() {
            let mut cities: Vec<Self> = data[key]
                .as_array().unwrap()
                .iter()
                .map(|el| City { title: el.as_str().unwrap().to_owned() })
                .collect();

            compare_cities.append(&mut cities);
        }
            
        Ok(compare_cities)
    }
}