use serenity::{
    model::application::interaction::application_command::ApplicationCommandInteraction,
    prelude::Context,
};

pub async fn verify(ctx: &Context, command: &ApplicationCommandInteraction) -> String {
    let mut member = command.clone().member.unwrap();
    let _ = member.add_role(&ctx.http, 785220867180724245).await;
    let _ = member.add_role(&ctx.http, 714157786824441886).await;
    let _ = member.remove_role(&ctx.http, 853098704063037460).await;
    let _ = command.clone().user.direct_message(&ctx, |message| {
        message.content(
            "Grimgar: Remastered (In-Dev):\n\nThank you for verifying! Don't forget to read rules also!",
        )
    }).await;
    "Verified!".into()
}
