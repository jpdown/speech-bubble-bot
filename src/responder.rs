use crate::database::entities::guild_config::Model;
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

    let db = crate::database::DbConn::new().await;
    let guild_config = match db.get_guild_config(guild_id.into()).await {
        Ok(gc) => match gc {
            Some(c) => c,
            None => {
                return;
            }
        },
        Err(e) => {
            println!("Error getting config for guild {guild_id}: {e}");
            return;
        }
    };

    let respond_rng: f64 = random();
    if respond_rng < guild_config.response_chance {
        match _new_message.reply(_ctx.http, "lmao").await {
            Ok(..) => {}
            Err(e) => println!("Error responding to message: {}", e),
        };
    }
}
