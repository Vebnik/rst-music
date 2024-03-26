mod test;
mod weather;

use crate::types::Data;

pub fn get_cmds() -> Vec<poise::Command<Data, Box<(dyn std::error::Error + std::marker::Send + Sync + 'static)>>> {
    vec![
        test::command::age(),
        weather::command::weather(),
    ]
}
