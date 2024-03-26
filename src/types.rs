use crate::error::Error;

pub struct Data {}

pub type Context<'a> = poise::Context<'a, Data, Error>;