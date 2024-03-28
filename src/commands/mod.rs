mod test;
mod weather;
mod music;

use crate::types::Data;

pub fn get_cmds() -> Vec<poise::Command<Data, Box<(dyn std::error::Error + std::marker::Send + Sync + 'static)>>> {
    vec![
        test::command::pagiante(),
        weather::command::weather(),
        music::command::play(),
    ]
}
