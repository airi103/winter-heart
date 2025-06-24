use poise::serenity_prelude as serenity;

use dotenv::dotenv;
use std::time::Instant;

mod commands;
use crate::commands::about::about;
use crate::commands::user_info::user_info;

struct Data {
    start_time: Instant,
} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let start_time = Instant::now();

    let token = std::env::var("DISCORD_TOKEN").expect("Missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler(ctx, event, framework, data))
            },
            commands: vec![user_info(), about()],
            ..Default::default()
        })
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                let guild_id = serenity::GuildId::new(
                    std::env::var("GUILD_ID")
                        .expect("Missing GUILD_ID")
                        .parse::<u64>()
                        .expect("Failed to parse GUILD_ID"),
                );
                poise::builtins::register_in_guild(ctx, &framework.options().commands, guild_id)
                    .await?;
                Ok(Data { start_time })
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}

async fn event_handler(
    _ctx: &serenity::Context, // marked
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    _data: &Data, // marked
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::Ready { data_about_bot, .. } => {
            println!(
                "Logged in as {}#{}",
                data_about_bot.user.name,
                data_about_bot.user.discriminator.unwrap()
            );
        }
        _ => {}
    }
    Ok(())
}
