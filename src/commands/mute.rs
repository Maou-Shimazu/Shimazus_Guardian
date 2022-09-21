use serenity::model::application::interaction::application_command::CommandDataOptionValue;
use serenity::model::prelude::interaction::InteractionResponseType;
use serenity::prelude::Mentionable;
use serenity::utils::Colour;
use serenity::{
    model::application::interaction::application_command::ApplicationCommandInteraction,
    prelude::Context,
};
use sqlx::{Pool, Sqlite};
use std::fs;

pub async fn new_case(
    pool: Pool<Sqlite>,
    moderator: String,
    reason: String,
    userid: i64,
) -> Result<(), sqlx::Error> {
    sqlx::query(&format!(
        r#"
    INSERT INTO cases ( action, moderator, reason, userid )
    VALUES ( mute, {moderator}, {reason}, {userid} )
            "#,
    ))
    .execute(&mut pool.acquire().await.unwrap())
    .await
    .unwrap()
    .last_insert_rowid();
    Ok(())
}

pub fn update_table() {}

pub async fn mute(ctx: &Context, command: &ApplicationCommandInteraction, pool: Pool<Sqlite>) {
    // todo: remove all roles from user before mute
    let u_user = command
        .data
        .options
        .get(0)
        .expect("Expected user option.")
        .resolved
        .as_ref()
        .expect("Expected user option.");
    let reason = command
        .data
        .options
        .get(1) // warning: makes it required, fix
        .expect("Expected reason")
        .resolved
        .as_ref()
        .expect("Expected reason");

    let reason = match reason {
        CommandDataOptionValue::String(result) => result,
        _ => "No Reason",
    };
    let mut r: String = String::new();
    if let CommandDataOptionValue::User(user, _member) = u_user {
        for i in _member.clone().unwrap().roles {
            match ctx
                .http
                .remove_member_role(command.guild_id.unwrap().0, user.id.0, i.0, Some(reason))
                .await
            {
                Ok(_) => log::info!("Removed role: {i}"),
                Err(e) => log::info!("Could not remove roles because: {e}"),
            }
        }
        match ctx
            .http
            .add_member_role(
                command.guild_id.unwrap().0,
                user.id.0,
                732986237832527982,
                Some(reason),
            )
            .await
        {
            Ok(_) => r = format!("<:Butler:895521263974494248> Muted: {}!", user.mention()),
            Err(e) => {
                r = format!(
                    "<:peepoDetective:803936363849842689> Could not mute {} because of: {e}",
                    user.mention()
                )
            }
        }
        update_table();
        // warning: add values
        new_case(pool, String::new(), String::new(), 1)
            .await
            .expect("Could not update mute case");
    }
    if let Err(why) = command
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| {
                    message.embed(|content| content.title(r).colour(Colour::DARK_GREEN))
                })
        })
        .await
    {
        log::error!("Cannot respond to slash command: {}", why);
    }
}
