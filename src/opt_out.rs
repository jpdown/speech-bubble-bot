use crate::{Context, Error};

#[poise::command(prefix_command, slash_command)]
pub async fn optout(ctx: Context<'_>) -> Result<(), Error> {
    let db = crate::database::DbConn::new().await;
    let opted_out = db.opt_out(ctx.author().id.into()).await?;

    if opted_out {
        ctx.say("Successfully opted out.").await?;
    } else {
        ctx.say("You are already opted out.").await?;
    }

    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn optin(ctx: Context<'_>) -> Result<(), Error> {
    let db = crate::database::DbConn::new().await;
    let opted_in_rows = db.opt_in(ctx.author().id.into()).await?;

    if opted_in_rows > 0 {
        ctx.say("Successfully opted in.").await?;
    } else {
        ctx.say("You are already opted in.").await?;
    }

    Ok(())
}
