use serenity::{
    model::{
        application::interaction::{
            application_command::ApplicationCommandInteraction, InteractionResponseType,
        },
        prelude::interaction::application_command::CommandDataOptionValue,
        user::User,
        Timestamp,
    },
    prelude::Context,
    utils::Colour,
};

use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;

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

pub async fn av(ctx: &Context, command: &ApplicationCommandInteraction) {
    let user: &CommandDataOptionValue;
    let mut id: User = command.member.clone().unwrap().user;
    let mut avatar: Option<String> =
        Some("https://cdn.discordapp.com/embed/avatars/0.png".to_string());

    if let Some(e) = command.data.options.get(0) {
        user = e.resolved.as_ref().expect("Expected user option.");
        if let CommandDataOptionValue::User(uuser, _) = user {
            avatar = uuser.avatar_url();
            id = uuser.clone();
        }
    } else {
        id = command.member.clone().unwrap().user;
        avatar = command.member.clone().unwrap().user.avatar_url();
    };

    if let Err(why) = command
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| {
                    message.embed(|content| {
                        content
                            .title("Avatar")
                            .author(|f| f.name(id.tag()).icon_url(id.avatar_url().unwrap()))
                            .image(avatar.unwrap())
                            .colour(Colour::BLURPLE)
                    })
                })
        })
        .await
    {
        log::error!("Cannot respond to slash command: {}", why);
    }
}
