use serenity::{
    model::{
        application::interaction::application_command::ApplicationCommandInteraction,
        prelude::interaction::application_command::CommandDataOptionValue,
    },
    prelude::Context,
};
pub async fn message(ctx: &Context, command: &ApplicationCommandInteraction) {
    let channel = command
        .data
        .options
        .get(0)
        .expect("Expected channel.")
        .resolved
        .as_ref()
        .expect("Expected channel.");
    let message = command
        .data
        .options
        .get(1)
        .expect("Expected message")
        .resolved
        .as_ref()
        .expect("Expected message");

    if let CommandDataOptionValue::Channel(c) = channel {
        if let CommandDataOptionValue::String(s) = message {
            c.id.send_message(&ctx.http, |msg| {
                msg.content("Content Title").embed(|embed| {
                    embed
                        .title("This is a title")
                        .description("This is a description")
                        .image("attachment://ferris_eyes.png")
                        .fields(vec![
                            ("This is the first field", "This is a field body", true),
                            ("This is the second field", "Both fields are inline", true),
                        ])
                        .field(
                            "This is the third field",
                            "This is not an inline field",
                            false,
                        )
                        .footer(|f| f.text("This is a footer"))
                })
            })
            .await
            .expect("could not send message");
        }
    }
}
