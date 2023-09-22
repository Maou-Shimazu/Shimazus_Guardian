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
use std::time::Duration;

use dotenv::dotenv;
use serenity::async_trait;
use serenity::builder::CreateButton;
use serenity::client::EventHandler;
use serenity::futures::StreamExt;
use serenity::model::application::component::ButtonStyle;
use serenity::model::prelude::*;
use serenity::prelude::*;

use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("help")
        .description("View available commands for Grimgar.")
        .create_option(|command| {
            command
                .name("command")
                .description("Command to check.")
                .kind(CommandOptionType::String)
                .required(false)
                .add_string_choice("av", "av")
                .add_string_choice("ban", "ban")
                .add_string_choice("help", "help")
                .add_string_choice("kick", "kick")
                .add_string_choice("message", "message")
                .add_string_choice("mute", "mute")
                .add_string_choice("ping", "ping")
                .add_string_choice("unban", "unban")
                .add_string_choice("unmute", "unmute")
                .add_string_choice("verify", "verify")
                .add_string_choice("whois", "whois")
        })
}

pub async fn help(ctx: &Context, command: &ApplicationCommandInteraction) {
    let cmd: &CommandDataOptionValue;
    if let Some(e) = command.data.options.get(0) {
        cmd = e.resolved.as_ref().expect("Expected user option.");
        if let CommandDataOptionValue::String(_command) = cmd {
            let emb = match _command.as_str() {
                "av" => vec![
                    ("Description", "Get a user's avatar", false),
                    ("Usage", "/av [user]", false),
                ],
                "ban" => vec![
                    ("Description", "Ban a user.", false),
                    ("Usage", "/ban <user> <reason>", false),
                ],
                "help" => vec![
                    (
                        "Description",
                        "Get information about all or a specific command.",
                        false,
                    ),
                    ("Usage", "/help [command]", false),
                ],
                "kick" => vec![
                    ("Description", "Kick a user.", false),
                    ("Usage", "/kick <user> <reason>", false),
                ],
                "message" => vec![
                    ("Description", "Send mod information with bot.", false),
                    ("Usage", "/message <message>", false),
                ],
                "mute" => vec![
                    ("Description", "Mute a user.", false),
                    ("Usage", "/mute <user> <reason>", false),
                ],
                "ping" => vec![
                    ("Description", "Get bot latency.", false),
                    ("Usage", "/ping", false),
                ],
                "unban" => vec![
                    ("Description", "Unban a user.", false),
                    ("Usage", "/unban <user> <reason>", false),
                ],
                "unmute" => vec![
                    ("Description", "Unmute a user.", false),
                    ("Usage", "/unmute <user>", false),
                ],
                "verify" => vec![
                    ("Description", "Verify a user in the server.", false),
                    ("Usage", "/verify", false),
                ],
                "whois" => vec![
                    ("Description", "Get a user's information", false),
                    ("Usage", "/whois [user]", false),
                ],
                _ => vec![("Invalid Command", "Please enter a valid command", false)],
            };
            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| {
                            message
                                .embed(|embed| {
                                    embed
                                        .title(&format!("/{_command} info"))
                                        .fields(emb)
                                        .color(Colour::from_rgb(47, 49, 54))
                                        .footer(|f| f.text("Usage Syntax: <required> [optional]"))
                                })
                                .ephemeral(true)
                        })
                })
                .await
            {
                log::error!("Cannot respond to slash command: {}", why);
            }
        }
    } else {
        let emb = vec![
            (
                "</av:1024113848397090906>",
                "<:reply:1025367487291867136> Get your's or another user's avatar.",
                false,
            ),
            (
                "</ban:1022959689916481681>",
                "<:reply:1025367487291867136> Ban a user.",
                false,
            ),
            (
                "</help:1022959690956681286>",
                "<:reply:1025367487291867136> You know what this does!",
                false,
            ),
            (
                "</kick:1022959689916481680>",
                "<:reply:1025367487291867136> Kick a user.",
                false,
            ),
            (
                "</message:1022959689916481679>",
                "<:reply:1025367487291867136> Send a message, using the bot, to a channel.",
                false,
            ),
            (
                "</mute:1022959689916481678>",
                "<:reply:1025367487291867136> Mute a user.",
                false,
            ),
            (
                "</ping:1022959689916481676>",
                "<:reply:1025367487291867136> Check bot's api latency.",
                false,
            ),
            (
                "</unban:1022959689916481682>",
                "<:reply:1025367487291867136> Unban a user.",
                false,
            ),
            (
                "</unmute:1022959689916481684>",
                "<:reply:1025367487291867136> Get your's or another user's avatar.",
                false,
            ),
            (
                "</verify:1022959689916481677>",
                "<:reply:1025367487291867136> Get your's or another user's avatar.",
                false,
            ),
            (
                "</whois:1022959689916481683>",
                "<:reply:1025367487291867136> Get your's or another user's avatar.",
                false,
            ),
        ];

        if let Err(why) = command
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| {
                        message
                            .embed(|embed| {
                                embed
                                    .description("All Available Commands for Cosmic Productions")
                                    .fields(emb)
                                    .color(Colour::from_rgb(47, 49, 54))
                            })
                            .ephemeral(true)
                    })
            })
            .await
        {
            log::error!("Cannot respond to slash command: {}", why);
        }
    }
}
