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
pub async fn kick(ctx: &Context, command: &ApplicationCommandInteraction) {
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
                .kick_member_with_reason(command.guild_id.unwrap().0, user.id.0, r)
                .await
            {
                Ok(_) => {
                    match user
                            .direct_message(&ctx.http, |e| {
                                e.embed(|dm| {
                                    dm.title(format!(
                                        "You have been kicked from Grimgar: Remastered for {r}",
                                    ))
                                })
                                // e.content(format!("You have been kicked from Grimgar: Remastered for {r}",))
                            })
                        .await
                    {
                        Ok(_) => (),
                        Err(e) => {
                            command
                                .channel_id
                                .send_message(&ctx.http, |msg| {
                                    msg.embed(|embed| {
                                        embed
                                            .description(format!("Could not dm {} because: ```{e}```", user.name))
                                            .color(Colour::RED)
                                    })
                                })
                                .await
                                .expect("Could not send kick falure message");
                        }
                    };
                    result = format!(
                        "<:Butler:895521263974494248> Kicked: ```{}```. Reason: ``{}``",
                        user.name, r
                    )
                }
                Err(e) => {
                    result = format!(
                        "<:peepoDetective:803936363849842689> Could not kick ```{}``` because of: ```{e}```",
                        user.name
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
                    message.embed(|content| content.title(result).colour(Colour::DARK_GREEN))
                })
        })
        .await
    {
        log::error!("Cannot respond to slash command: {}", why);
    }
}
