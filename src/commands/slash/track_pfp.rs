use base64::encode;
use serenity::builder::CreateApplicationCommand;
use serenity::model::application::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue,
};
use serenity::model::user::User;

use crate::database::queries;
use crate::handler::handler::Handler;

pub async fn run(options: &[CommandDataOption], handler: &Handler) -> String {
    let target_option = match options.get(0) {
        Some(option) => option,
        None => return "Missing required user option".to_string(),
    };

    let resolved = match target_option.resolved.as_ref() {
        Some(resolved) => resolved,
        None => return "Could not resolve selected user".to_string(),
    };

    if let CommandDataOptionValue::User(user, member) = resolved {
        let display_name = member
            .as_ref()
            .and_then(|partial_member| partial_member.nick.clone())
            .unwrap_or_else(|| user.name.clone());
        match track_user_avatar_snapshot(user, &display_name, handler, true).await {
            Ok(true) => format!("Tracked profile picture for {}.", display_name),
            Ok(false) => format!("{} has no avatar change to store.", display_name),
            Err(err) => format!("Failed to track profile picture: {}", err),
        }
    } else {
        "Please provide a valid user.".to_string()
    }
}

pub async fn track_user_avatar_snapshot(
    user: &User,
    display_name: &str,
    handler: &Handler,
    force_check: bool,
) -> Result<bool, String> {
    let discord_user_id = user.id.to_string();
    let avatar_url = user
        .avatar_url()
        .unwrap_or_else(|| user.default_avatar_url());

    let already_tracked = queries::is_user_tracked(&discord_user_id, handler)
        .await
        .map_err(|err| format!("database error checking tracked user: {}", err))?;

    if !already_tracked && !force_check {
        return Ok(false);
    }

    queries::upsert_tracked_user(&discord_user_id, display_name, handler)
        .await
        .map_err(|err| format!("database error upserting tracked user: {}", err))?;

    let last_avatar = queries::latest_tracked_avatar_url(&discord_user_id, handler)
        .await
        .map_err(|err| format!("database error reading latest tracked avatar: {}", err))?;

    if matches!(last_avatar, Some(last) if last == avatar_url) {
        return Ok(false);
    }

    let avatar_bytes = reqwest::get(&avatar_url)
        .await
        .map_err(|err| format!("failed to download avatar image: {}", err))?
        .bytes()
        .await
        .map_err(|err| format!("failed to read avatar image bytes: {}", err))?;
    let avatar_base64 = encode(&avatar_bytes);

    queries::insert_tracked_pfp(
        &discord_user_id,
        display_name,
        &avatar_url,
        &avatar_base64,
        handler,
    )
    .await
    .map_err(|err| format!("database error inserting avatar snapshot: {}", err))?;

    Ok(true)
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("track_pfp")
        .description("Track and store a user's profile picture")
        .create_option(|option| {
            option
                .name("user")
                .description("User to track")
                .kind(CommandOptionType::User)
                .required(true)
        })
}
