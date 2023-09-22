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
use sqlx::{Pool, Sqlite};

use crate::core::{
    cases::{new_case, Moderation},
    log::modlog,
};
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::Permissions;

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("ban")
        .description("Ban a member.")
        .default_member_permissions(Permissions::BAN_MEMBERS)
        .create_option(|channel| {
            channel
                .name("user")
                .description("User to Ban.")
                .kind(CommandOptionType::User)
                .required(true)
        })
        .create_option(|message| {
            message
                .name("reason")
                .description("Reason for Ban.")
                .kind(CommandOptionType::String)
                .required(true)
        })
        .create_option(|days| {
            days.name("days")
                .description("Delete message days.")
                .kind(CommandOptionType::Integer)
                .required(true)
        })
}

pub async fn ban(ctx: &Context, command: &ApplicationCommandInteraction, pool: Pool<Sqlite>) {
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
        .get(1)
        .expect("Expected reason")
        .resolved
        .as_ref()
        .expect("Expected reason");
    let days = command
        .data
        .options
        .get(2)
        .expect("Expected days")
        .resolved
        .as_ref()
        .expect("Expected days");

    let moderator = command.member.clone().unwrap().user.id.0;
    let mut result: String = String::new();
    if let CommandDataOptionValue::User(user, _member) = u_user {
        if let CommandDataOptionValue::String(r) = reason {
            match user
                .direct_message(&ctx.http, |e| {
                    e.embed(|dm| {
                        dm.description(format!(
                            "You have been banned from Grimgar: Remastered | {r}",
                        ))
                    })
                })
                .await
            {
                Ok(_) => (),
                Err(e) => {
                    command
                        .channel_id
                        .send_message(&ctx.http, |msg| {
                            msg.embed(|embed| {
                                embed
                                    .description(format!("Could not dm {} | {e}", user.tag()))
                                    .color(Colour::RED)
                            })
                        })
                        .await
                        .expect("Could not send ban falure message");
                }
            };
            let days = match days {
                CommandDataOptionValue::Integer(r) => r.to_owned() as u8,
                _ => 0,
            };
            match ctx
                .http
                .ban_user(command.guild_id.unwrap().0, user.id.0, days, r)
                .await
            {
                Ok(_) => {
                    result = format!(
                        "<:Butler:895521263974494248> ***{} was banned*** | {}",
                        user.tag(),
                        r
                    )
                }
                Err(e) => {
                    result = format!(
                        "<:peepoDetective:803936363849842689> ***Could not ban {}*** | {e}",
                        user.tag()
                    )
                }
            };
            let id = new_case(pool.clone(), Moderation::Ban, moderator, r, user.id.0)
                .await
                .expect("Could not update ban case");
            modlog(&ctx, &pool, id)
                .await
                .expect("Could not send to modlog");
        }
    }
    if let Err(why) = command
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| {
                    message.embed(|content| content.description(result).colour(Colour::DARK_GREEN))
                })
        })
        .await
    {
        log::error!("Cannot respond to slash command: {}", why);
    }
}
