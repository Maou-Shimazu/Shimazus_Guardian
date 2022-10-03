use serenity::builder::CreateApplicationCommand;
use serenity::model::Permissions;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue,
};

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("message")
        .description("Send a moderator message.")
        .default_member_permissions(Permissions::MANAGE_GUILD)
        .create_option(|channel| {
            channel
                .name("channel")
                .description("Channel to send message in")
                .kind(CommandOptionType::Channel)
                .required(true)
        })
        .create_option(|message| {
            message
                .name("message")
                .description("message to send")
                .kind(CommandOptionType::String)
                .required(true)
        })
}
