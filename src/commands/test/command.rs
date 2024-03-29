use crate::error::Error;
use crate::types::Context;

// Displays your or another user's account creation date
// #[poise::command(slash_command)]
// pub async fn pagiante(ctx: Context<'_>) -> Result<(), Error> {
//     let pages = &[
//         "Content of first page",
//         "Content of second page",
//         "Content of third page",
//         "Content of fourth page",
//     ];

//     poise::samples::paginate(ctx, pages).await?;

//     Ok(())
// }

#[poise::command(
    slash_command,
    subcommands("child1", "child2"),
    subcommand_required
)]
// Omit 'ctx' parameter here. It is not needed, because this function will never be called.
// TODO: Add a way to remove 'ctx' parameter, when `subcommand_required` is set
pub async fn parent_subcommand_required(_: Context<'_>) -> Result<(), Error> {
    // This will never be called, because `subcommand_required` parameter is set
    Ok(())
}

/// A subcommand of `parent`
#[poise::command(prefix_command, slash_command)]
pub async fn child1(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("You invoked the first child command!").await?;
    Ok(())
}

/// Another subcommand of `parent`
#[poise::command(prefix_command, slash_command)]
pub async fn child2(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("You invoked the second child command!").await?;
    Ok(())
}