use poise::serenity_prelude::{Context, Message};
use rand::prelude::*;

pub async fn on_message(_ctx: Context, _new_message: Message) {
    if _new_message.author.bot {
        return;
    }

    let respond_rng: f32 = random();
    if respond_rng < 0.5 {
        match _new_message.reply(_ctx.http, "50% chance").await {
            Ok(..) => {}
            Err(e) => println!("Error responding to message: {}", e),
        };
    }
}
