mod commands;
mod database;
mod guild_config;
mod responder;

use poise::serenity_prelude::Message;
use poise::{async_trait, serenity_prelude as serenity};
use std::time::Duration;

// Custom user data passed to all command functions
pub struct Data {}

// All command functions use these types
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    // This is our custom error handler
    // They are many errors that can occur, so we only handle the ones we want to customize
    // and forward the rest to the default handler
    match error {
        poise::FrameworkError::Setup { error, .. } => panic!("Failed to start bot: {:?}", error),
        poise::FrameworkError::Command { error, ctx } => {
            println!("Error in command `{}`: {:?}", ctx.command().name, error,);
            let _ = ctx.say("Error running command. Why did you do that??").await;
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                println!("Error while handling error: {}", e)
            }
        }
    }
}

struct Handler;

#[async_trait]
impl serenity::EventHandler for Handler {
    async fn message(&self, _ctx: serenity::Context, _new_message: Message) {
        responder::on_message(_ctx, _new_message).await;
    }
}

#[tokio::main]
async fn main() {
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::age(),
                commands::help(),
                guild_config::chance(),
                guild_config::add(),
                guild_config::remove(),
            ],
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some(";".into()),
                edit_tracker: Some(poise::EditTracker::for_timespan(Duration::from_secs(3600))),
                ..Default::default()
            },
            on_error: |error| Box::pin(on_error(error)),
            ..Default::default()
        })
        .token(std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN"))
        .intents(
            serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT,
        )
        .client_settings(|client| client.event_handler(Handler))
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                println!(
                    "Logged in as {} in {} guilds",
                    _ready.user.name,
                    _ready.guilds.len()
                );
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                // Initialize custom data here
                Ok(Data {})
            })
        });

    framework.run().await.unwrap();
}
