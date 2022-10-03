use serenity::builder::CreateApplicationCommand;
use serenity::model::Permissions;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue,
};

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("unmute")
        .description("Unmute a user")
        .default_member_permissions(Permissions::MUTE_MEMBERS)
        .create_option(|user| {
            user.name("user")
                .description("User to unmute")
                .kind(CommandOptionType::User)
                .required(true)
        })
}
