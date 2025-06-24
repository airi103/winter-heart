use humantime::format_duration;

use crate::{Context, Error};

use poise::{CreateReply, serenity_prelude as serenity};
use serenity::{
    Colour, ReactionType,
    builder::{CreateActionRow, CreateButton, CreateEmbed, CreateInteractionResponseMessage},
    model::prelude::{EmojiId, Timestamp},
};

/// Returns information about the bot
#[poise::command(prefix_command, slash_command)]
pub async fn about(ctx: Context<'_>) -> Result<(), Error> {
    let uptime = ctx.data().start_time.elapsed();

    let reply = ctx.send(build_response(uptime)).await?;

    loop {
        match reply
            .message()
            .await?
            .await_component_interaction(ctx)
            .author_id(ctx.author().id)
            .timeout(std::time::Duration::from_secs(60))
            .await
        {
            Some(interaction) => {
                let new_uptime = ctx.data().start_time.elapsed();

                interaction
                    .create_response(
                        ctx,
                        serenity::CreateInteractionResponse::UpdateMessage(
                            build_interaction_response(new_uptime),
                        ),
                    )
                    .await?;
            }
            None => {
                let new_uptime = ctx.data().start_time.elapsed();

                let disabled_reply = build_disabled_reply(new_uptime);

                reply.edit(ctx, disabled_reply).await?;

                break;
            }
        }
    }
    Ok(())
}

fn build_disabled_reply(uptime: std::time::Duration) -> CreateReply {
    let embed = build_embed(uptime);
    CreateReply::default()
        .embed(embed)
        .components(vec![CreateActionRow::Buttons(vec![
            CreateButton::new("refresh_about")
                .label("Refresh")
                .emoji(ReactionType::Custom {
                    animated: false,
                    id: EmojiId::new(1276212568318414848),
                    name: None,
                })
                .style(serenity::ButtonStyle::Primary)
                .disabled(true),
            CreateButton::new_link("https://discord.gg/xbGwxfN8vu").label("Support Server"),
        ])])
}

fn build_embed(uptime: std::time::Duration) -> CreateEmbed {
    let embed = CreateEmbed::new()
        .title("Winter Heart Statistics")
        .description("Here is some information about me!")
        .field("Uptime", format_duration(uptime).to_string(), true)
        .field("Serenity Version", "0.12.4", true)
        .field("Bot Version", env!("CARGO_PKG_VERSION"), true)
        .colour(Colour::BLURPLE)
        .timestamp(Timestamp::from_unix_timestamp(Timestamp::now().timestamp()).unwrap());
    embed
}

fn build_response(uptime: std::time::Duration) -> CreateReply {
    let embed = build_embed(uptime);
    let reply = CreateReply::default()
        .embed(embed)
        .components(vec![CreateActionRow::Buttons(vec![
            CreateButton::new("refresh_about")
                .label("Refresh")
                .emoji(ReactionType::Custom {
                    animated: false,
                    id: EmojiId::new(1276212568318414848),
                    name: None,
                })
                .style(serenity::ButtonStyle::Primary),
            CreateButton::new_link("https://discord.gg/xbGwxfN8vu").label("Support Server"),
        ])]);
    reply
}

fn build_interaction_response(uptime: std::time::Duration) -> CreateInteractionResponseMessage {
    let embed = build_embed(uptime);
    let reply = CreateInteractionResponseMessage::new()
        .embed(embed)
        .components(vec![CreateActionRow::Buttons(vec![
            CreateButton::new("refresh_about")
                .label("Refresh")
                .emoji(ReactionType::Custom {
                    animated: false,
                    id: EmojiId::new(1276212568318414848),
                    name: None,
                })
                .style(serenity::ButtonStyle::Primary),
            CreateButton::new_link("https://discord.gg/xbGwxfN8vu").label("Support Server"),
        ])]);
    reply
}
