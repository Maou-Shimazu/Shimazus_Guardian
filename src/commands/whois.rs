use serenity::{
    model::{
        application::interaction::{
            application_command::ApplicationCommandInteraction, InteractionResponseType,
        },
        prelude::interaction::application_command::CommandDataOptionValue,
        Timestamp,
    },
    prelude::{Context, Mentionable},
    utils::Colour,
};

pub async fn whois(ctx: &Context, command: &ApplicationCommandInteraction) {
    let user = command.user.clone();
    let mut result: String = String::new();
    match command
        .channel_id
        .send_message(&ctx.http, |message| {
            message.embed(|embed| {
                embed
                    // .set_author(author)
                    .url(user.face())
                    .thumbnail(user.avatar_url().unwrap())
                    .description(format!("@{}", user.tag()))
                    .field("Registered", user.created_at(), true)
            })
        })
        .await
    {
        Ok(_) => {}
        Err(e) => {
            result = format!(
                "<:peepoDetective:803936363849842689> ***Could not send information.*** | {e}"
            )
        }
    };
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
