use serenity::model::application::interaction::application_command::CommandDataOptionValue;
use serenity::prelude::Mentionable;
use serenity::{
    model::application::interaction::application_command::ApplicationCommandInteraction,
    prelude::Context,
};
pub async fn mute<'a>(ctx: &Context, command: &ApplicationCommandInteraction) -> &'a str {
    // todo: remove all roles from user before mute
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
        .get(1) // warning: makes it required, fix
        .expect("Expected reason")
        .resolved
        .as_ref()
        .expect("Expected reason");

    let reason = match reason {
        CommandDataOptionValue::String(result) => result,
        _ => "No Reason",
    };
    let r: &str;
    if let CommandDataOptionValue::User(user, _member) = u_user {
        for i in _member.clone().unwrap().roles {
            match ctx
                .http
                .remove_member_role(command.guild_id.unwrap().0, user.id.0, i.0, Some(reason))
                .await
            {
                Ok(_) => log::info!("Removed role: {i}"),
                Err(e) => log::info!("Could not remove roles because: {e}"),
            }
        }
        match ctx
            .http
            .add_member_role(
                command.guild_id.unwrap().0,
                user.id.0,
                732986237832527982,
                Some(reason),
            )
            .await
        {
            Ok(_) => r = &format!("Muted: {}!", user.mention()),
            Err(e) => r = &format!("Could not {} user because of: {e}", user.mention()),
        }
    } else {
        r = "Could not mute user.".into()
    }
    // r
    "mute test"
}
