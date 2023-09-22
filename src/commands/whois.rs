use serenity::{
    model::{
        application::interaction::{
            application_command::ApplicationCommandInteraction, InteractionResponseType,
        },
        prelude::interaction::application_command::CommandDataOptionValue,
        user::User,
        Permissions, Timestamp,
    },
    prelude::{Context, Mentionable},
    utils::Colour,
};

use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;

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

pub async fn whois(ctx: &Context, command: &ApplicationCommandInteraction) {
    let mut _user: &CommandDataOptionValue;
    let mut user: User = command.user.clone();

    if let Some(e) = command.data.options.get(0) {
        _user = e.resolved.as_ref().expect("Expected user option.");
        if let CommandDataOptionValue::User(uuser, _) = _user {
            user = uuser.clone();
        }
    }

    let guser = ctx
        .http
        .get_member(command.guild_id.unwrap().0, user.id.0)
        .await
        .unwrap();
    let joined = guser.clone().joined_at.unwrap();
    let (roles, role_count) = (guser.clone().roles, guser.clone().roles.len());
    let mut _roles: String = String::new();
    roles
        .iter()
        .for_each(|f| _roles.push_str(&format!("<@&{}> ", f.0)));

    let _permissions = command
        .clone()
        .member
        .unwrap()
        .permissions
        .unwrap()
        .get_permission_names();

    let mut perms: String = String::new(); // warning: fix key permissions
    _permissions.iter().for_each(|f| match *f {
        "Administrator" => perms.push_str("Administrator, "),
        "Manage Guilds" => perms.push_str("Manage Guilds, "),
        "Manage Roles" => perms.push_str("Manage Roles, "),
        "Mention Everyone" => perms.push_str("Mention Everyone, "),
        "Manage Webhooks" => perms.push_str("Manage Webhooks, "),
        "Manage Nicknames" => perms.push_str("Manage Nicknames, "),
        "Ban Members" => perms.push_str("Ban Members, "),
        "Change Nickname" => perms.push_str("Change Nickname, "),
        "Manage Emojis and Stickers" => perms.push_str("Manage Emojis and Stickers, "),
        "Manage Channels" => perms.push_str("Manage Channels, "),
        "Kick Members" => perms.push_str("Kick Members, "),
        "Manage Messages" => perms.push_str("Manage Messages"),
        _ => (),
    });
    if perms.is_empty() {
        perms = String::from("None")
    }
    if let Err(why) = command
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| {
                    message.embed(|embed| {
                        embed
                            .author(|f| f.name(user.tag()).icon_url(user.avatar_url().unwrap()))
                            .url(user.face())
                            .description(user.mention())
                            .thumbnail(user.avatar_url().unwrap())
                            .field("Joined", format!("<t:{}:R>", joined.timestamp()), true)
                            .field(
                                "Registered",
                                format!("<t:{}:R>", user.created_at().timestamp()),
                                true,
                            )
                            .field(format!("Roles[{role_count}]",), _roles, false)
                            .field("Key Permissions", perms, false)
                            .field("ID", user.id, true)
                            .field(
                                "Sent",
                                format!("<t:{}:R>", Timestamp::now().timestamp()),
                                true,
                            )
                            .colour(Colour::FADED_PURPLE)
                    })
                })
        })
        .await
    {
        log::error!("Cannot respond to slash command: {}", why);
    }
}
