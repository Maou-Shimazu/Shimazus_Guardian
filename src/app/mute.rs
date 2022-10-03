use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue,
};
use serenity::model::Permissions;

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("mute")
        .description("Mute a user")
        .default_member_permissions(Permissions::MUTE_MEMBERS)
        .create_option(|user| {
            user.name("user")
                .description("User to Mute")
                .kind(CommandOptionType::Mentionable)
                .required(true)
        })
        .create_option(|time| {
            time.name("time")
                .description("Time (Minutes) for mute command to last")
                .kind(CommandOptionType::Integer)
                .required(true)
        })
        .create_option(|reason| {
            reason
                .name("reason")
                .description("Reason for muting")
                .kind(CommandOptionType::String)
                .required(true)
        })
}
