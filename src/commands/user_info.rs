use crate::{Context, Error};

use poise::{CreateReply, serenity_prelude as serenity};

use serenity::model::prelude::Timestamp;

/// Returns info about the user
#[poise::command(slash_command, prefix_command)]
pub async fn user_info(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let g = ctx.guild_id().unwrap();
    let member = g.member(&ctx.serenity_context().http, u.id).await?;

    let created_timestamp = &u.created_at().unix_timestamp();
    let joined_timestamp = member.joined_at.unwrap().unix_timestamp();

    let roles = &member.roles;

    ctx.send(
        CreateReply::default().embed(
            serenity::CreateEmbed::new()
                .title(format!("{}'s Info", u.name))
                .fields(vec![
                    (
                        "Username",
                        &format!("{} ({})", &u.name, &u.display_name()),
                        true,
                    ),
                    ("ID", &u.id.get().to_string(), true),
                    (
                        "Account created",
                        &format!("<t:{}:F> (<t:{}:R>)", created_timestamp, created_timestamp),
                        true,
                    ),
                    (
                        "Joined Server",
                        &format!("<t:{}:F> (<t:{}:R>)", joined_timestamp, joined_timestamp),
                        true,
                    ),
                    (
                        &format!("Roles [{}]", roles.len()),
                        &roles
                            .iter()
                            .map(|id| format!("<@&{id}> "))
                            .collect::<String>(),
                        false,
                    ),
                ])
                .colour(member.colour(&ctx.cache()).unwrap())
                .thumbnail(&u.avatar_url().unwrap())
                .timestamp(Timestamp::from_unix_timestamp(Timestamp::now().timestamp()).unwrap()),
        ),
    )
    .await?;
    Ok(())
}
