use crate::{auth::HasAuth, responses::create_request_log_message, s3bucket::delete, Context};
use poise::serenity_prelude as serenity;

#[poise::command(slash_command)]
pub async fn request(
    ctx: Context<'_>,
    #[description = "Image"] file: serenity::Attachment,
) -> anyhow::Result<()> {
    let created_message_link = create_request_log_message(ctx, file.url).await?;
    ctx.send(
        poise::CreateReply::default()
            .content(format!("Created Request: {}", created_message_link))
            .ephemeral(true),
    )
    .await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn remove(
    ctx: Context<'_>,
    #[description = "User id to remove: Leave blank to remove your own"] user: Option<String>,
) -> anyhow::Result<()> {
    match user {
        Some(user) => {
            if ctx
                .author()
                .has_auth(ctx.serenity_context(), ctx.data())
                .await?
                || ctx.author().id.to_string() == user
            {
                delete(ctx.data(), user.clone()).await?;
                ctx.send(
                    poise::CreateReply::default()
                        .content(format!("Removed Banner for {}", user))
                        .ephemeral(true),
                )
                .await?;
            } else {
                ctx.send(
                    poise::CreateReply::default()
                        .content("You do not have authorization to remove another user's banner")
                        .ephemeral(true),
                )
                .await?;
            }
        }
        None => {
            delete(ctx.data(), ctx.author().id.to_string()).await?;
            ctx.send(
                poise::CreateReply::default()
                    .content("Removed Your Banner")
                    .ephemeral(true),
            )
            .await?;
        }
    }
    Ok(())
}
