use serenity::client::Context;
use poise::serenity_prelude::{Message, ChannelId};

pub struct MessageLink {
    server_id: u64,
    channel_id: u64,
    message_id: u64,
}

impl TryFrom<&str> for MessageLink {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut link_parts = value.split('/').skip(4);

        let server_id = link_parts
            .next()
            .ok_or("Invalid link")?
            .parse::<u64>()
            .map_err(|_| "Invalid server id")?;
        let channel_id = link_parts
            .next()
            .ok_or("Invalid link")?
            .parse::<u64>()
            .map_err(|_| "Invalid channel id")?;
        let message_id = link_parts
            .next()
            .ok_or("Invalid link")?
            .parse::<u64>()
            .map_err(|_| "Invalid message id")?;

        Ok(Self {
            server_id,
            channel_id,
            message_id, 
        })
    }
}

pub async fn handle(ctx: Context, new_message: Message) {
    let mut parts = new_message.content.split(' ');
    // Skip the command name
    parts.next();

    let link = match parts.next() {
        Some(link) => link,
        None => return,
    };

    let Ok(link) = MessageLink::try_from(link) else { return };
    let channel_id = ChannelId(link.channel_id);
    let content = parts.fold(String::new(), |output, part| output + part + " ");

    let message = match channel_id.message(&ctx.http, link.message_id).await {
        Ok(m) => m,
        Err(_) => return 
    };

    message.reply(&ctx.http, content).await.unwrap();
}