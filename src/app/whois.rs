use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue,
};

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("whois")
        .description("Information about a user.")
        .create_option(|user| {
            user.name("user")
                .description("User's information to view.")
                .kind(CommandOptionType::User)
                .required(false)
        })
}
