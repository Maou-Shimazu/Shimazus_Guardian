use serenity::builder::CreateApplicationCommand;
use serenity::model::Permissions;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue,
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
