mod test;
mod weather;

use crate::types::Data;

pub fn get_cmds() -> Vec<poise::Command<Data, Box<(dyn std::error::Error + std::marker::Send + Sync + 'static)>>> {
    vec![
        test::command::pagiante(),
        weather::command::weather(),
    ]
}
