use serenity::builder::CreateApplicationCommand;
use serenity::model::Permissions;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue,
};

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
