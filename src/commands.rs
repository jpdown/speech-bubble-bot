use crate::{Context, Error};
use serenity::model::prelude::User;

/// Show this help menu
#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help about"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<String>,
) -> Result<(), Error> {
    poise::builtins::help(
        ctx,
        command.as_deref(),
        poise::builtins::HelpConfiguration {
            ephemeral: true,
            extra_text_at_bottom: "I exist to call people out when they say something dumb",
            ..Default::default()
        },
    )
    .await?;
    Ok(())
}
