use crate::database::DbConn;
use poise::serenity_prelude::{Context, Message};
use rand::prelude::*;
use sea_orm::DbErr;

pub async fn on_message(_ctx: Context, _new_message: Message) {
    if _new_message.author.bot {
        return;
    }

    let guild_id = match _new_message.guild_id {
        Some(id) => id,
        None => {
            return;
        }
    };

    let db = DbConn::new().await;

    let opted_out = match db.get_opted_out(_new_message.author.id.into()).await {
        Ok(opt_out) => opt_out,
        Err(e) => {
            println!("Error checking if user is opted out: {}", e);
            return;
        }
    };
    if opted_out {
        return;
    }

    let send = match should_send(&db, guild_id.into()).await {
        Ok(send) => send,
        Err(e) => {
            println!("Error checking if should send response: {}", e);
            return;
        }
    };

    if !send {
        return;
    }

    let image = db.get_random_image(guild_id.into()).await;

    if let Ok(Some(i)) = image {
        if let Err(e) = _new_message.reply(_ctx.http, i).await {
            println!("Error responding to message: {}", e);
        }
    };
}

async fn should_send(db: &DbConn, guild_id: i64) -> Result<bool, DbErr> {
    let guild_config = match db.get_guild_config(guild_id).await? {
        Some(c) => c,
        None => {
            return Ok(false);
        }
    };

    Ok(random::<f64>() < guild_config.response_chance)
}
