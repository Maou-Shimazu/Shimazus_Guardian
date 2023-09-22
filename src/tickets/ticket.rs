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
        .name("ticket")
        .description("Crate a ticket")
        .create_option(|f| {
            f.name("topic")
                .description("Topic for ticket.")
                .kind(CommandOptionType::String)
                .required(true)
        })
}

pub async fn ticket(ctx: &Context, command: &ApplicationCommandInteraction) {
    let mut msg = String::new();
    let topic = command
        .data
        .options
        .get(0)
        .unwrap()
        .resolved
        .as_ref()
        .expect("Expected user option.");
    if let CommandDataOptionValue::String(topic) = topic {
        msg = topic.clone();
    }

    let message = ctx
        .http
        .get_channel(1028429569423855636)
        .await
        .unwrap()
        .id()
        .send_message(&ctx.http, |f| {
            f.content(format!("Ticket: {} - {}", msg, command.user.tag()))
        })
        .await
        .expect("Could not send message");

    let map: Value = json!({
        "name": format!("Ticket {}", command.user.tag()),
        "type": 12,
        "auto_archive_duration": 60,
    });
    let map: JsonMap = map.as_object().unwrap().clone();

    let thread = ctx
        .http
        .create_public_thread(1028429569423855636, message.id.0, &map)
        .await
        .expect("Could not create thread");

    for i in [command.user.id.0, 763091467077943367, 409201896839184388] {
        ctx.http
            .add_thread_channel_member(thread.id.0, i)
            .await
            .expect("Could not add member to thread");
    }

    if let Err(why) = command
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| {
                    message
                        .content(format!("You can get help here: <#{}>", thread.id))
                        .ephemeral(true)
                })
        })
        .await
    {
        log::error!("Cannot respond to slash command: {}", why);
    }

    // make a command that closes and tries to export the chat
    // if thread.thread_metadata.unwrap().

    // thread
    //     .send_message(&ctx.http, |f| {
    //         f.embed(|f| f.description("Thank you for contacting support!"))
    //     })
    //     .await
    //     .expect("Could not send message").react(&ctx.http, "fv");
}
