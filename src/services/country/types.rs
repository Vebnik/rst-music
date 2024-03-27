use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct City {
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Country {
    pub title: String,
    pub cities: Vec<City>
}