#![allow(unused_imports)]
use serenity::async_trait;
use serenity::model::application::command::{Command, CommandOptionType};
use serenity::model::application::interaction::{
    application_command::CommandDataOptionValue, Interaction, InteractionResponseType,
};
use serenity::model::{
    channel::Message, gateway::Ready, id::ChannelId, id::GuildId, prelude::Member, Timestamp,
};
use serenity::prelude::*;
use std::env;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            log::info!("Received command interaction: {:#?}", command);

            // Name and content of interactions.
            let content: String = match command.data.name.as_str() {
                "ping" => "Pong!".into(),
                "help" => "".into(),
                "verify" => {
                    let mut member = command.clone().member.unwrap();
                    let _ = member.add_role(&ctx.http, 785220867180724245).await;
                    let _ = member.add_role(&ctx.http, 714157786824441886).await;
                    let _ = member.remove_role(&ctx.http, 853098704063037460).await;
                    "Verified!".into()
                }
                _ => "Unimplimented".into(),
            };

            // Respond to slash command with message content or log error of fail.
            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                log::error!("Cannot respond to slash command: {}", why);
            }
        }
    }
    async fn ready(&self, ctx: Context, ready: Ready) {
        log::info!("{} is connected!", ready.user.name);

        let guild_id = GuildId(
            env::var("GUILD_ID")
                .expect("Expected GUILD_ID in environment")
                .parse()
                .expect("GUILD_ID must be an integer"),
        );

        let commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands
                .create_application_command(|command| {
                    command.name("ping").description("A ping command")
                })
                .create_application_command(|command| {
                    command.name("verify").description("Verify your account")
                })
        })
        .await;

        log::info!("Guild slash commands: {:#?}", commands);

        let guild_command = Command::create_global_application_command(&ctx.http, |command| {
            command
                .name("help")
                .description("View available commands for Grimgar.")
        })
        .await;

        log::info!("Global slash command: {:#?}", guild_command);
    }

    async fn guild_member_addition(&self, ctx: Context, new_member: Member) {
        log::info!(
            "New member {}: {new_member} joined.",
            new_member.display_name()
        );
        let channel_id = ChannelId(713816140987629588);
        let _ = channel_id
            .send_message(&ctx.http, |m| {
                m.content("Welcome!")
                    .embed(|e| {
                        e.title("Grimgar Remastered")
                            .description(format!("Hello {new_member} and Welcome to Grimgar Remastered a game founded by Endless Drip we hope you enjoy your stay!
                            "))
                            .image("https://images-ext-2.discordapp.net/external/q2tJ6Y-Gj00VahghxYzgiRBzcEbhbWGcSX2G5JWMjpY/https/i.imgur.com/wZsxDv6.gif")
                            .timestamp(Timestamp::now())
                    })
            })
            .await;
    }
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    dotenv::dotenv().ok();
    log::info!("Starting Client.");

    let token = env::var("TOKEN").expect("Couldnt get Token.");
    /*GatewayIntents::non_privileged()
    | GatewayIntents::MESSAGE_CONTENT
    | GatewayIntents::GUILD_MESSAGES
    | GatewayIntents::DIRECT_MESSAGES*/
    let intents = GatewayIntents::all();
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        log::error!("An error occurred while running the client: {:?}", why);
    } else {
        log::info!("Client Ready.");
    }
}
