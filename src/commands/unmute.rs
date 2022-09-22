use async_std::task;
use serenity::model::application::interaction::application_command::CommandDataOptionValue;
use serenity::model::prelude::interaction::InteractionResponseType;
use serenity::prelude::Mentionable;
use serenity::utils::Colour;
use serenity::{
    model::application::interaction::application_command::ApplicationCommandInteraction,
    prelude::Context,
};
use sqlx::{Pool, Sqlite};
use std::fs;
use std::time::Duration;

pub async fn unmute(ctx: &Context, command: &ApplicationCommandInteraction, pool: Pool<Sqlite>) {
    // todo: remove all roles from user before mute
    let u_user = command
        .data
        .options
        .get(0)
        .expect("Expected user option.")
        .resolved
        .as_ref()
        .expect("Expected user option.");
}
