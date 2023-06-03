use crate::{guild_config, Context, Error};
use serenity::model::prelude::User;

#[poise::command(
    prefix_command,
    slash_command,
    required_permissions = "MANAGE_GUILD",
    guild_only = true
)]
pub async fn set_chance(ctx: Context<'_>, chance: u32) -> Result<(), Error> {
    let chance: f64 = 1.0 / f64::from(chance);

    let db = crate::database::DbConn::new().await;
    db.set_guild_chance(ctx.guild_id().unwrap().into(), chance)
        .await?;

    ctx.say("response chance set").await?;

    Ok(())
}
