use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue,
};

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("help")
        .description("View available commands for Grimgar.")
        .create_option(|command| {
            command
                .name("command")
                .description("Command to check.")
                .kind(CommandOptionType::String)
                .required(false)
                .add_string_choice("av", "av")
                .add_string_choice("ban", "ban")
                .add_string_choice("help", "help")
                .add_string_choice("kick", "kick")
                .add_string_choice("message", "message")
                .add_string_choice("mute", "mute")
                .add_string_choice("ping", "ping")
                .add_string_choice("unban", "unban")
                .add_string_choice("unmute", "unmute")
                .add_string_choice("verify", "verify")
                .add_string_choice("whois", "whois")
        })
}
