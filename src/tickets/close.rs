use std::collections::HashMap;

use async_std::stream::Map;
use serenity::{
    json::{hashmap_to_json_map, json, JsonMap, Value},
    model::{
        application::interaction::{
            application_command::ApplicationCommandInteraction, InteractionResponseType,
        },
        prelude::interaction::application_command::CommandDataOptionValue,
        user::User,
        Permissions, Timestamp,
    },
    prelude::{Context, Mentionable},
    utils::Colour,
};

use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("close")
        .description("Close a ticket")
        .default_member_permissions(Permissions::MANAGE_THREADS)
        .create_option(|f| {
            f.name("export")
                .description("Export contents of ticket.")
                .kind(CommandOptionType::Boolean)
                .required(true)
        })
}

pub async fn close(ctx: &Context, command: &ApplicationCommandInteraction) {
    let mut _user: &CommandDataOptionValue;
    let mut user: User = command.user.clone();

    if let Some(e) = command.data.options.get(0) {
        _user = e.resolved.as_ref().expect("Expected user option.");
        if let CommandDataOptionValue::User(uuser, _) = _user {
            user = uuser.clone();
        }
    }

    if let Err(why) = command
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| message)
        })
        .await
    {
        log::error!("Cannot respond to slash command: {}", why);
    }
}
