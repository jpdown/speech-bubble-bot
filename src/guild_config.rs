use crate::{Context, Error};

#[poise::command(
    prefix_command,
    slash_command,
    required_permissions = "BAN_MEMBERS",
    guild_only = true
)]
pub async fn chance(ctx: Context<'_>, chance: f64) -> Result<(), Error> {
    let db = crate::database::DbConn::new().await;

    db.set_guild_chance(ctx.guild_id().unwrap().into(), chance)
        .await?;

    ctx.say(format!("Response chance set to {}%", chance))
        .await?;

    Ok(())
}

#[poise::command(
    prefix_command,
    slash_command,
    required_permissions = "BAN_MEMBERS",
    guild_only = true
)]
pub async fn getchance(ctx: Context<'_>) -> Result<(), Error> {
    let db = crate::database::DbConn::new().await;

    let model = db.get_guild_config(ctx.guild_id().unwrap().into()).await?;

    let chance = match model {
        Some(m) => m.response_chance,
        _ => 0.0,
    };

    ctx.say(format!("Current response chance is {}%", chance))
        .await?;
    Ok(())
}

#[poise::command(
    prefix_command,
    slash_command,
    required_permissions = "BAN_MEMBERS",
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
    required_permissions = "BAN_MEMBERS",
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
