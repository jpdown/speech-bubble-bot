use crate::{guild_config, Context, Error};
use serenity::model::prelude::User;

use sea_orm::sea_query::ColumnSpec::Default;
use sea_orm::*;

#[poise::command(
    prefix_command,
    slash_command,
    required_permissions = "MANAGE_GUILD",
    guild_only = true
)]
pub async fn set_chance(ctx: Context<'_>, chance: usize) -> Result<(), Error> {
    let guild_config = guild_config::ActiveModel {
        ..Default.default()
    };
}
