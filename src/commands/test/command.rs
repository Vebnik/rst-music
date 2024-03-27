use crate::error::Error;
use crate::types::Context;

/// Displays your or another user's account creation date
#[poise::command(slash_command)]
pub async fn pagiante(ctx: Context<'_>) -> Result<(), Error> {
    let pages = &[
        "Content of first page",
        "Content of second page",
        "Content of third page",
        "Content of fourth page",
    ];

    poise::samples::paginate(ctx, pages).await?;

    Ok(())
}