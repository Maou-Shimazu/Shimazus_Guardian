#![allow(unused_imports)]
use serenity::async_trait;
use serenity::builder::CreateEmbed;
use serenity::model::prelude::{Embed, Guild};
use serenity::model::{
    application::{
        command::{Command, CommandOptionType},
        interaction::{
            application_command::CommandDataOptionValue, Interaction, InteractionResponseType,
        },
    },
    channel::Message,
    gateway::Ready,
    id::{ChannelId, GuildId},
    permissions::Permissions,
    prelude::{Member, ResumedEvent},
    Timestamp,
};
use serenity::prelude::*;
use sqlx::Sqlite;
use sqlx::{sqlite::SqlitePool, Pool};
use std::env;
mod app;
mod commands;
mod core;
use crate::commands::*;

async fn pool() -> Result<Pool<Sqlite>, sqlx::Error> {
    Ok(SqlitePool::connect("sqlite:main.sqlite").await?)
}

enum Content<'a> {
    String(&'a str),
    Embed(()),
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            log::info!("Received command interaction: {:#?}", command);

            // Name and content of interactions.
            let content: Content = match command.data.name.as_str() {
                "ping" => Content::String("Pong!"),
                "help" => Content::Embed(help::help(&ctx, &command).await),
                "verify" => Content::String(verify::verify(&ctx, &command).await),
                "mute" => Content::Embed(
                    mute::mute(
                        &ctx,
                        &command,
                        pool().await.expect("Expected database connection"),
                    )
                    .await,
                ),
                "message" => Content::Embed(message::message(&ctx, &command).await),
                "kick" => Content::Embed(kick::kick(&ctx, &command).await),
                "ban" => Content::Embed(
                    ban::ban(
                        &ctx,
                        &command,
                        pool().await.expect("Expected database connection"),
                    )
                    .await,
                ),
                "unban" => Content::Embed(unban::unban(&ctx, &command).await),
                "whois" => Content::Embed(whois::whois(&ctx, &command).await),
                "unmute" => Content::Embed(
                    unmute::unmute(
                        &ctx,
                        &command,
                        pool().await.expect("Expected database connection"),
                    )
                    .await,
                ),
                "av" => Content::Embed(av::av(&ctx, &command).await),
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
                .create_application_command(|command| app::ping::register(command))
                .create_application_command(|command| app::verify::register(command))
                .create_application_command(|command| app::mute::register(command))
                .create_application_command(|command| app::message::register(command))
                .create_application_command(|command| app::kick::register(command))
                .create_application_command(|command| app::ban::register(command))
                .create_application_command(|command| app::unban::register(command))
                .create_application_command(|command| app::whois::register(command))
                .create_application_command(|command| app::unmute::register(command))
                .create_application_command(|command| app::av::register(command))
                .create_application_command(|command| app::help::register(command))
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

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // note: add an event for listening to case updates in [`moderation`].
    // note: send mod embeds about each case update.
    env::set_var("RUST_LOG", "grimgar");
    pretty_env_logger::init();
    dotenv::dotenv().ok();
    log::info!("Starting Client.");

    let _initialize_query =
        sqlx::query(&std::fs::read_to_string("tables.sql").expect("could not open file"))
            .execute(&mut pool().await?.acquire().await?)
            .await?
            .last_insert_rowid();

    let token = env::var("TOKEN").expect("Couldnt get Token.");
    let intents = GatewayIntents::all();
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        log::error!("An error occurred while running the client: {:?}", why);
    }
    Ok(())
}
