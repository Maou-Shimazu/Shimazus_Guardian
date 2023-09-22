use serenity::{
    model::application::interaction::application_command::ApplicationCommandInteraction,
    prelude::Context,
};

use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue,
};

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("verify").description("Verify your account")
}


pub async fn verify<'a>(ctx: &Context, command: &ApplicationCommandInteraction) -> &'a str {
    let mut member = command.clone().member.unwrap();
    let _ = member.add_role(&ctx.http, 785220867180724245).await;
    let _ = member.add_role(&ctx.http, 714157786824441886).await;
    let _ = member.remove_role(&ctx.http, 853098704063037460).await;
    let _ = command.clone().user.direct_message(&ctx, |message| {
        message.content(
            "Grimgar: Remastered (In-Dev):\n\nThank you for verifying! Don't forget to read rules also!",
        )
    }).await;
    "Verified!"
}
