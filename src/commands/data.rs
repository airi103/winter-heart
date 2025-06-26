use crate::Data;
use crate::db::Item;
use crate::db::read_inventory;
use crate::db::update_inventory;
use crate::{Context, Error};
use humantime::format_duration;
use poise::CreateReply;
use poise::serenity_prelude::CreateEmbed;
use poise::serenity_prelude::User;
use serenity::all::Colour;
use serenity::all::Timestamp;

// TODO: Save cooldown state in database
/// Get a daily check-in reward!
#[poise::command(
    prefix_command,
    slash_command,
    user_cooldown = 86400,
    on_error = "handle_cooldown"
)]
pub async fn daily(ctx: Context<'_>) -> Result<(), Error> {
    update_inventory(ctx.author().id.get(), Item::SnowGlobe, 1, &ctx.data().pool).await?;

    ctx.send(
        CreateReply::default().embed(
            CreateEmbed::new()
                .title("Daily Check-in")
                .description(format!(
                    "[1] {} Snow Globe was placed in your inventory!",
                    Item::SnowGlobe.emoji()
                ))
                .timestamp(Timestamp::now())
                .colour(Colour::BLURPLE),
        ),
    )
    .await?;

    Ok(())
}

async fn handle_cooldown(error: poise::FrameworkError<'_, Data, Error>) {
    if let poise::FrameworkError::CooldownHit {
        remaining_cooldown,
        ctx,
        ..
    } = error
    {
        ctx.send(
            CreateReply::default()
                .content(format!(
                    "You've already checked-in today.\nTry again in {}.",
                    format_duration(remaining_cooldown)
                ))
                .ephemeral(true),
        )
        .await;
    } else {
        // fallback
        poise::builtins::on_error(error).await;
    }
}

#[poise::command(prefix_command, slash_command)]
pub async fn inventory(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let v = read_inventory(u.id.get(), &ctx.data().pool).await?;

    let embed = CreateEmbed::new()
        .title(format!("{}'s Inventory", u.name))
        .timestamp(Timestamp::now())
        .colour(Colour::BLURPLE);

    let embed = v.iter().fold(embed, |embed, (item, quantity)| {
        embed.field(
            format!("{} {}", item.emoji(), item.to_string()),
            quantity.to_string(),
            false,
        )
    });

    ctx.send(CreateReply::default().embed(embed)).await?;

    Ok(())
}
