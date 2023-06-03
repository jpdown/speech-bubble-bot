use crate::{guild_config, Context, Error};
use serenity::model::prelude::User;

#[poise::command(
    prefix_command,
    slash_command,
    required_permissions = "MANAGE_GUILD",
    guild_only = true
)]
pub async fn chance(ctx: Context<'_>, chance: u32) -> Result<(), Error> {
    let chance: f64 = 1.0 / f64::from(chance);

    let db = crate::database::DbConn::new().await;
    db.set_guild_chance(ctx.guild_id().unwrap().into(), chance)
        .await?;

    ctx.say(format!("Response chance set to {}%", chance * 100.0)).await?;

    Ok(())
}

#[poise::command(
    prefix_command,
    slash_command,
    required_permissions = "MANAGE_GUILD",
    guild_only = true
)]
pub async fn add(ctx: Context<'_>, url: String) -> Result<(), Error> {
    let db = crate::database::DbConn::new().await;
    let res = db.add_image(ctx.guild_id().unwrap().into(), url).await?;

    if res {
        ctx.say("url added").await?;
    } else {
        ctx.say("url already exists").await?;
    }

    Ok(())
}

#[poise::command(
    prefix_command,
    slash_command,
    required_permissions = "MANAGE_GUILD",
    guild_only = true
)]
pub async fn remove(ctx: Context<'_>, url: String) -> Result<(), Error> {
    let db = crate::database::DbConn::new().await;
    let res = db.remove_image(ctx.guild_id().unwrap().into(), url).await?;

    if res > 0 {
        ctx.say("url removed").await?;
    } else {
        ctx.say("url does not exist").await?;
    }

    Ok(())
}
