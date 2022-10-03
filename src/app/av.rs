use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue,
};

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("av")
        .description("Get a user's avatar.")
        .create_option(|user| {
            user.name("user")
                .description("User to unmute")
                .kind(CommandOptionType::User)
                .required(false)
        })
}
