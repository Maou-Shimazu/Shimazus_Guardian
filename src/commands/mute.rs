use async_std::task;
use serenity::model::application::interaction::application_command::CommandDataOptionValue;
use serenity::model::prelude::interaction::InteractionResponseType;
use serenity::model::prelude::RoleId;
use serenity::prelude::Mentionable;
use serenity::utils::Colour;
use serenity::{
    model::application::interaction::application_command::ApplicationCommandInteraction,
    prelude::Context,
};
use sqlx::{Pool, Sqlite};
use std::fs;
use std::time::Duration;

pub async fn new_case(
    pool: Pool<Sqlite>,
    moderator: u64,
    reason: &str,
    userid: u64,
) -> Result<(), sqlx::Error> {
    sqlx::query(&format!(
        r#"
    INSERT INTO cases ( action, moderator_id, reason, userid )
    VALUES ( "mute", {moderator}, "{reason}", {userid} )
            "#,
    ))
    .execute(&mut pool.acquire().await?)
    .await?
    .last_insert_rowid();
    Ok(())
}

pub async fn muted(pool: Pool<Sqlite>, userid: u64, roles: Vec<RoleId>) -> Result<(), sqlx::Error> {
    let mut temp: Vec<String> = vec![];
    roles.iter().for_each(|i| {
        temp.push(i.0.to_string());
    });
    let role: String = temp.join(" ");
    log::info!("id: {userid}, roles: {role}");
    sqlx::query(&format!(
        r#"
        INSERT INTO muted ( userid, roles )
        VALUES ( {userid}, "{role}" )"#,
    ))
    .execute(&mut pool.acquire().await?)
    .await?
    .last_insert_rowid();
    Ok(())
}

pub async fn mute(ctx: &Context, command: &ApplicationCommandInteraction, pool: Pool<Sqlite>) {
    let u_user = command
        .data
        .options
        .get(0)
        .expect("Expected user option.")
        .resolved
        .as_ref()
        .expect("Expected user option.");

    let time = command
        .data
        .options
        .get(1)
        .expect("Expected reason")
        .resolved
        .as_ref()
        .expect("Expected reason");

    let reason = command
        .data
        .options
        .get(2)
        .expect("Expected reason")
        .resolved
        .as_ref()
        .expect("Expected reason");

    // warning: collect id instead
    let moderator = command.member.clone().unwrap().user.id.0;

    let reason = match reason {
        CommandDataOptionValue::String(result) => result,
        _ => "No Reason",
    };
    let mut r: String = String::new();
    if let CommandDataOptionValue::User(user, _member) = u_user {
        muted(pool.clone(), user.id.0, _member.clone().unwrap().roles)
            .await
            .expect("Couldnt update muted table");
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
            Ok(_) => {
                r = format!(
                    "<:Butler:895521263974494248> Muted: {} | {reason}",
                    user.tag()
                )
            }
            Err(e) => {
                r = format!(
                    "<:peepoDetective:803936363849842689> Could not mute {} because of: {e}",
                    user.tag()
                )
            }
        }
        // muted();
        new_case(pool.clone(), moderator, reason, user.id.0)
            .await
            .expect("Could not update mute case");
    }
    if let CommandDataOptionValue::Integer(t) = time.clone() {
        log::info!("Starting unmute timer");
        let temp = ctx.clone();
        let cmd = command.clone();
        if let CommandDataOptionValue::User(user, _member) = u_user.clone() {
            task::spawn(async move {
                // note: reason and default reason
                log::info!("Sleeping for {t} minutes");
                task::sleep(Duration::from_secs((t * 60) as u64)).await;
                log::info!("Finished sleeping, unmuting user.");
                temp.http
                    .remove_member_role(
                        cmd.guild_id.unwrap().0,
                        user.id.0,
                        732986237832527982,
                        Some("Unmuting"),
                    )
                    .await
                    .expect("Could not remove muted role");
                let roles = crate::commands::unmute::get_roles(&pool, user.id.0 as i64)
                    .await
                    .expect("Could not get roles");
                let vroles: Vec<u64> = roles
                    .split(" ")
                    .map(|x| x.parse::<u64>().unwrap())
                    .collect();

                log::info!("roles: {vroles:?}");
                for role in vroles {
                    temp.http
                        .add_member_role(cmd.guild_id.unwrap().0, user.id.0, role, Some("Unmuting"))
                        .await
                        .expect("Could not return role from unmute");
                }
                crate::commands::unmute::drop_muted_info(&pool, user.id.0 as i64)
                    .await
                    .expect("Could not drop user from muted.");
            });
        }
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
