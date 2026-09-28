use base64::encode;
use serenity::builder::CreateApplicationCommand;
use serenity::model::application::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue,
};

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
        let avatar_url = user
            .avatar_url()
            .unwrap_or_else(|| user.default_avatar_url());
        let display_name = member
            .as_ref()
            .and_then(|partial_member| partial_member.nick.clone())
            .unwrap_or_else(|| user.name.clone());
        let discord_user_id = user.id.to_string();
        let avatar_bytes = match reqwest::get(&avatar_url).await {
            Ok(response) => match response.bytes().await {
                Ok(bytes) => bytes,
                Err(err) => return format!("Failed to read avatar image bytes: {}", err),
            },
            Err(err) => return format!("Failed to download avatar image: {}", err),
        };
        let avatar_base64 = encode(&avatar_bytes);

        match queries::insert_tracked_pfp(
            &discord_user_id,
            &display_name,
            &avatar_url,
            &avatar_base64,
            handler,
        )
        .await
        {
            Ok(_) => format!("Tracked profile picture for {}.", display_name),
            Err(err) => format!("Failed to track profile picture: {}", err),
        }
    } else {
        "Please provide a valid user.".to_string()
    }
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
