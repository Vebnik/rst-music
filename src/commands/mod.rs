mod test;
mod weather;
mod music;

use crate::types::Data;

pub fn get_cmds() -> Vec<poise::Command<Data, Box<(dyn std::error::Error + std::marker::Send + Sync + 'static)>>> {
    vec![
        test::command::parent_subcommand_required(),
        weather::command::weather(),
        // music scope command
        music::command::play(),
        music::command::leave(),
    ]
}
