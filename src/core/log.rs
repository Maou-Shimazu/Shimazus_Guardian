use serenity::{
    model::{
        application::interaction::{
            application_command::ApplicationCommandInteraction, InteractionResponseType,
        },
        prelude::interaction::application_command::CommandDataOptionValue,
        Timestamp,
    },
    prelude::Context,
    utils::Colour,
};
use sqlx::{Pool, Sqlite, SqlitePool};

pub async fn modlog(ctx: &Context, pool: &SqlitePool, id: i64) -> Result<(), sqlx::Error> {
    let table = sqlx::query!(
        "
        SELECT * 
        FROM cases 
        WHERE id = ?",
        id
    )
    .fetch_one(pool)
    .await?;

    let user = ctx
        .http
        .get_user(table.userid.clone() as u64)
        .await
        .unwrap();

    let color = match table.action.clone().as_str() {
        "Mute" => Colour::from_rgb(255, 71, 15),
        "Ban" => Colour::from_rgb(240, 74, 71),
        "Unban" => Colour::from_rgb(250, 219, 94),
        "Kick" => Colour::from_rgb(240, 74, 71),
        "Warn" => Colour::from_rgb(250, 219, 94),
        "UpdatedWarn" => Colour::from_rgb(255, 71, 15),
        "Warnings" => Colour::from_rgb(255, 71, 15),
        "Lock" => Colour::from_rgb(255, 71, 15),
        "Unlock" => Colour::from_rgb(255, 71, 15),
        "Unmute" => Colour::from_rgb(67, 181, 130),
        _ => Colour::BLURPLE,
    };

    ctx.http
        .get_channel(784982385112514600)
        .await
        .unwrap()
        .id()
        .send_message(&ctx.http, |msg| {
            msg.embed(|embed| {
                embed
                    .author(|f| {
                        f.name(&format!(
                            "Case {id} | {} | {}",
                            table.action.clone(),
                            user.tag()
                        ))
                        .icon_url(user.avatar_url().unwrap())
                    })
                    .fields(vec![
                        ("User", format!("<@{}>", table.userid.clone()), true),
                        (
                            "Moderator",
                            format!("<@{}>", table.moderator_id.clone()),
                            true,
                        ),
                        ("Reason", table.reason.clone(), true),
                    ])
                    .footer(|footer| footer.text(format!("ID: {}", table.userid.clone())))
                    .timestamp(Timestamp::now())
                    .color(color)
            })
        })
        .await
        .expect("Couldnt send log message.");
    Ok(())
}
