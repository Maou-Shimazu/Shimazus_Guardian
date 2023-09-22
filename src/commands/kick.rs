use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue,
};
use serenity::model::Permissions;
use serenity::{
    model::{
        application::interaction::{
            application_command::ApplicationCommandInteraction, InteractionResponseType,
        },
        Timestamp,
    },
    prelude::Context,
    utils::Colour,
};

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("kick")
        .description("Kick a member.")
        .default_member_permissions(Permissions::KICK_MEMBERS)
        .create_option(|channel| {
            channel
                .name("user")
                .description("User to kick.")
                .kind(CommandOptionType::User)
                .required(true)
        })
        .create_option(|message| {
            message
                .name("reason")
                .description("Reason for kick.")
                .kind(CommandOptionType::String)
                .required(true)
        })
}

pub async fn kick(ctx: &Context, command: &ApplicationCommandInteraction) {
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

    let mut result: String = String::new();
    if let CommandDataOptionValue::User(user, _member) = u_user {
        if let CommandDataOptionValue::String(r) = reason {
            match user
                .direct_message(&ctx.http, |e| {
                    e.embed(|dm| {
                        dm.description(format!(
                            "You have been kicked from Grimgar: Remastered | {r}",
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
                        .expect("Could not send kick falure message");
                }
            };
            match ctx
                .http
                .kick_member_with_reason(command.guild_id.unwrap().0, user.id.0, r)
                .await
            {
                Ok(_) => {
                    result = format!(
                        "<:Butler:895521263974494248> ***{} was kicked*** | {}",
                        user.tag(),
                        r
                    )
                }
                Err(e) => {
                    result = format!(
                        "<:peepoDetective:803936363849842689> ***Could not kick {}*** | {e}",
                        user.tag()
                    )
                }
            };
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
