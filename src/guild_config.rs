use serenity::builder::CreateEmbed;
use crate::{Context, Error};

#[poise::command(
    prefix_command,
    slash_command,
    required_permissions = "BAN_MEMBERS",
    guild_only = true
)]
pub async fn chance(ctx: Context<'_>, chance: f64) -> Result<(), Error> {
    let db = crate::database::DbConn::new().await;

    db.set_guild_chance(ctx.guild_id().unwrap().into(), chance / 100.0)
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
        Some(m) => m.response_chance * 100.0,
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

#[poise::command(
prefix_command,
slash_command,
required_permissions = "BAN_MEMBERS",
guild_only = true
)]
pub async fn responses(ctx: Context<'_>) -> Result<(), Error> {
    let db = crate::database::DbConn::new().await;
    let images = db.get_all_images(ctx.guild_id().unwrap().into()).await?;

    let left_id = format!("{}_prev", ctx.id());
    let right_id = format!("{}_next", ctx.id());

    let pages: Vec<CreateEmbed> = images.chunks(4).map(|c| {
        let embed = CreateEmbed::default()
            .title("Response Images")
            .colour((0xc3, 0x80, 0xff));

        for img in c {
            embed.image(img);
        }

        return embed.to_owned();
    }).collect();

    let message = ctx.send(|m| {
        m.components(|c| {
            c.create_action_row(|r| {
                r.create_button(|b| b.custom_id(&left_id).emoji('◀'))
                    .create_button(|b| b.custom_id(&right_id).emoji('▶'))
            })
        }).reply(true)
            .ephemeral(true)
    });

    Ok(())
}
