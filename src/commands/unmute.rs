use async_std::task;
use serenity::model::application::interaction::application_command::CommandDataOptionValue;
use serenity::model::prelude::interaction::InteractionResponseType;
use serenity::prelude::Mentionable;
use serenity::utils::Colour;
use serenity::{
    model::application::interaction::application_command::ApplicationCommandInteraction,
    prelude::Context,
};
use sqlx::{Pool, Row, Sqlite, SqlitePool};
use std::fs;
use std::time::Duration;

/// Get the roles from the muted table. Query: 
/// ```sql
/// SELECT roles FROM muted WHERE userid = ?1
/// ```
pub async fn get_roles(pool: &SqlitePool, id: i64) -> Result<String, sqlx::Error> {
    let res = sqlx::query!(
        r#"
        SELECT roles
        FROM muted
        WHERE userid = ?1
        "#,
        id
    )
    .fetch_one(pool)
    .await?;
    Ok(res.roles.unwrap())
}

/// Drop user from the table when they are unmuted Query: 
/// ```sql
/// DELETE FROM muted WHERE userid = ?1
/// ```
pub async fn drop_muted_info(pool: &SqlitePool, userid: i64) -> Result<(), sqlx::Error> {
    sqlx::query!("DELETE FROM muted WHERE userid = ?1", userid)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn unmute(ctx: &Context, command: &ApplicationCommandInteraction, pool: Pool<Sqlite>) {
    // warning: check if user has mute role
    // note: add reason
    let u_user = command
        .data   
        .options
        .get(0)
        .expect("Expected user option.")
        .resolved
        .as_ref()
        .expect("Expected user option.");

    let mut r: String = String::new();
    if let CommandDataOptionValue::User(user, _member) = u_user {
        match ctx
            .http
            .remove_member_role(
                command.guild_id.unwrap().0,
                user.id.0,
                732986237832527982,
                Some("Unmuting"),
            )
            .await
        {
            Ok(_) => r = format!("<:Butler:895521263974494248> Unmuted: {}!", user.tag()),
            Err(e) => {
                r = format!(
                    "<:peepoDetective:803936363849842689> Could not unmute {} because of: {e}",
                    user.tag()
                )
            }
        }

        let roles = get_roles(&pool, user.id.0 as i64)
            .await
            .expect("Could not get roles.");
        let vroles: Vec<u64> = roles
            .split(" ")
            .map(|x| x.parse::<u64>().unwrap())
            .collect();

        log::info!("roles: {vroles:?}");
        for role in vroles {
            ctx.http
                .add_member_role(
                    command.guild_id.unwrap().0,
                    user.id.0,
                    role,
                    Some("Unmuting"),
                )
                .await
                .expect("Could not return role from unmute");
        }
        drop_muted_info(&pool, user.id.0 as i64)
            .await
            .expect("Could not drop user from muted.");
    }
    if let Err(why) = command
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| {
                    message.embed(|content| content.description(r).colour(Colour::DARK_GREEN))
                })
        })
        .await
    {
        log::error!("Cannot respond to slash command: {}", why);
    }
}
