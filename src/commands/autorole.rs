use johnny::{apply_embed, create_embed, Context, Error};
use poise::serenity_prelude::{ButtonStyle, Guild, InteractionResponseType, ReactionType};
use serenity::{
    builder::{CreateButton, CreateComponents},
    futures::StreamExt,
};
use std::time::Duration;

#[allow(dead_code)]
fn button_id(guild: &Guild) -> String {
    format!("autorole-{}", guild.id)
}

#[allow(dead_code)]
fn toggle_button(guild: &Guild, enabled: &bool) -> CreateButton {
    let mut toggle_button = CreateButton::default();

    toggle_button
        .custom_id(button_id(guild))
        .emoji(ReactionType::Unicode(
            if !enabled { "✅" } else { "❌" }.to_string(),
        ))
        .label(if !enabled { "Enable" } else { "Disable" }.to_string() + " autorole")
        .style(if !enabled {
            ButtonStyle::Success
        } else {
            ButtonStyle::Danger
        });

    toggle_button
}

/// View or modify current autorole settings
#[poise::command(
    slash_command,
    default_member_permissions = "MANAGE_GUILD",
    required_bot_permissions = "MANAGE_ROLES",
    guild_only
)]
pub async fn autorole(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;

    let mut enabled = false; // todo: this should be in the database
    let guild = ctx.guild().unwrap(); // command is guild only so unwrap is safe
    let mut embed = create_embed(&ctx).await;

    embed
        .title(format!("{} - autorole", guild.name))
        .description("make meaniningful description soon (:"); // todo: do it.

    // create the toggle button

    // send the message
    let mut reply = ctx
        .send(|msg| {
            apply_embed(msg, &embed).components(|components| {
                components.create_action_row(|row| row.add_button(toggle_button(&guild, &enabled)))
            })
        })
        .await?
        .into_message()
        .await?;

    // todo: add role selector

    // wait for a response
    let mut interaction_stream = reply
        .await_component_interactions(ctx)
        .timeout(Duration::from_secs(60 * 3))
        .build();

    while let Some(interaction) = interaction_stream.next().await {
        // check if toggle button was pressed
        if interaction.data.custom_id == button_id(&guild) {
            enabled = !enabled;

            interaction
                .create_interaction_response(&ctx, |response| {
                    response
                        .kind(InteractionResponseType::DeferredUpdateMessage)
                        .interaction_response_data(|data| {
                            data.add_embed(embed.title("hi").clone())
                                .components(|components| {
                                    components.create_action_row(|row| {
                                        row.add_button(toggle_button(&guild, &enabled))
                                    })
                                })
                        })
                })
                .await
                .expect("should have been able to respond to autorole toggle interaction");
        }

        ctx.data()
            .logger
            .info(format!("that is {:?}", enabled), Some(&ctx))
            .await;
    }

    // remove dangling components
    reply
        .edit(&ctx, |message| {
            message.set_components(CreateComponents::default())
        })
        .await?;

    Ok(())
}
