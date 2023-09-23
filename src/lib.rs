#![allow(unused_imports)]
use anyhow::anyhow;
use serenity::async_trait;
use serenity::builder::CreateEmbed;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::prelude::{Embed, Guild};
use serenity::model::{
    application::{
        command::{Command, CommandOptionType},
        interaction::{
            application_command::CommandDataOptionValue, Interaction, InteractionResponseType,
        },
    },
    id::{ChannelId, GuildId},
    permissions::Permissions,
    prelude::{Member, ResumedEvent},
    Timestamp,
};
use serenity::prelude::*;
use serenity::prelude::*;
use shuttle_secrets::SecretStore;
use sqlx::Sqlite;
use sqlx::{sqlite::SqlitePool, Pool};
use std::env;
use tracing::{error, info};
mod commands;
mod core;
mod tickets;
use crate::commands::*;

#[allow(dead_code)]
struct Bot {
    database: SqlitePool,
    guild_id: String,
}

enum Content<'a> {
    String(&'a str),
    Embed(()),
}

#[async_trait]
impl EventHandler for Bot {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            log::info!("Received command interaction: {:#?}", command);

            // Name and content of interactions.
            let content: Content = match command.data.name.as_str() {
                "ping" => Content::String("Pong!"),
                "help" => Content::Embed(help::help(&ctx, &command).await),
                "verify" => Content::String(verify::verify(&ctx, &command).await),
                "mute" => Content::Embed(mute::mute(&ctx, &command, self.database.clone()).await),
                "message" => Content::Embed(message::message(&ctx, &command).await),
                "kick" => Content::Embed(kick::kick(&ctx, &command).await),
                "ban" => Content::Embed(ban::ban(&ctx, &command, self.database.clone()).await),
                "unban" => Content::Embed(unban::unban(&ctx, &command).await),
                "whois" => Content::Embed(whois::whois(&ctx, &command).await),
                "unmute" => {
                    Content::Embed(unmute::unmute(&ctx, &command, self.database.clone()).await)
                }
                "av" => Content::Embed(av::av(&ctx, &command).await),
                "ticket" => Content::Embed(tickets::ticket::ticket(&ctx, &command).await),
                "close" => Content::Embed(tickets::close::close(&ctx, &command).await),
                _ => Content::String("Unimplimented"),
            };

            // Respond to slash command with message content or log error of fail.
            match content {
                Content::String(message_response) => {
                    if let Err(why) = command
                        .create_interaction_response(&ctx.http, |response| {
                            response
                                .kind(InteractionResponseType::ChannelMessageWithSource)
                                .interaction_response_data(|message| {
                                    message.content(message_response)
                                })
                        })
                        .await
                    {
                        log::error!("Cannot respond to slash command: {}", why);
                    }
                }
                Content::Embed(_) => {}
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
                // .create_application_command(|command| ping::register(command))
                .create_application_command(|command| verify::register(command))
                .create_application_command(|command| mute::register(command))
                .create_application_command(|command| message::register(command))
                .create_application_command(|command| kick::register(command))
                .create_application_command(|command| ban::register(command))
                .create_application_command(|command| unban::register(command))
                .create_application_command(|command| whois::register(command))
                .create_application_command(|command| unmute::register(command))
                .create_application_command(|command| av::register(command))
                .create_application_command(|command| help::register(command))
                .create_application_command(|command| tickets::ticket::register(command))
                .create_application_command(|command| tickets::close::register(command))
        })
        .await;

        log::info!("Guild slash commands: {:#?}", commands);

        // let guild_command =
        // Command::create_global_application_command(&ctx.http, |command| {}).await;

        // log::info!("Global slash command: {:#?}", guild_command);
    }
    async fn resume(&self, _: Context, _: ResumedEvent) {
        log::info!("Resumed");
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

#[shuttle_service::main]
async fn serenity(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_service::ShuttleSerenity {
    // Get the discord token set in `Secrets.toml`
    env::set_var("RUST_LOG", "grimgar");
    pretty_env_logger::init();
    dotenv::dotenv().ok();
    log::info!("Starting Client.");

    let token = secret_store.get("DISCORD_TOKEN").unwrap();
    let intents = GatewayIntents::all();
    let bot = Bot {
        database: SqlitePool::connect("sqlite:main.sqlite").await.unwrap(),
        guild_id: secret_store.get("GUILD_ID").unwrap(),
    };
    let client = Client::builder(&token, intents)
        .event_handler(bot)
        .await
        .expect("Err creating client");

    Ok(client)
}
