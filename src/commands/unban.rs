use serenity::{
    model::{
        application::interaction::{
            application_command::ApplicationCommandInteraction, InteractionResponseType,
        },
        prelude::interaction::application_command::CommandDataOptionValue,
        Timestamp,
    },
    prelude::Context,
    utils::Colour,
};
pub async fn unban(ctx: &Context, command: &ApplicationCommandInteraction) {
    let u_user = command
        .data
        .options
        .get(0)
        .expect("Expected user option.")
        .resolved
        .as_ref()
        .expect("Expected user option.");
    let reason = command
        .data
        .options
        .get(1)
        .expect("Expected reason")
        .resolved
        .as_ref()
        .expect("Expected reason");

    let mut result: String = String::new();
    if let CommandDataOptionValue::User(user, _member) = u_user {
        if let CommandDataOptionValue::String(r) = reason {
            match ctx
                .http
                .remove_ban(command.guild_id.unwrap().0, user.id.0, Some(r))
                .await
            {
                Ok(_) => {
                    result = format!(
                        "<:Butler:895521263974494248> ***{} was unbanned*** | {}",
                        user.tag(),
                        r
                    )
                }
                Err(e) => {
                    result = format!(
                        "<:peepoDetective:803936363849842689> ***Could not unban {}*** | {e}",
                        user.tag()
                    )
                }
            };
        }
    }
    if let Err(why) = command
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| {
                    message.embed(|content| content.description(result).colour(Colour::DARK_GREEN))
                })
        })
        .await
    {
        log::error!("Cannot respond to slash command: {}", why);
    }
}
