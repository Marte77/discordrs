use serenity::builder::CreateApplicationCommand;
use serenity::model::application::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{CommandDataOption,CommandDataOptionValue};

pub fn run(_options: &[CommandDataOption]) -> String {
    /*let nome_opt = match _options.get(0){
        None => CommandDataOptionValue::String("erro".to_owned()),
        Some(n) => match n.resolved.as_ref() {
            None =>CommandDataOptionValue::String("erro".to_owned()),
            Some(s) => s.to_owned()
        }
    };
    if let CommandDataOptionValue::String(nome) = nome_opt {
        format!("ola amigui: {}", nome)
    }else{
        format!("ola amigui: erro")
    }*/
    let option = _options
        .get(0)
        .expect("Expected attachment option")
        .resolved
        .as_ref()
        .expect("Expected attachment object");

    if let CommandDataOptionValue::String(attachment) = option {
        format!("ola amigui: {}", attachment)
    } else {
        "Please provide a valid attachment".to_string()
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("ola")
        //.name_localized("en_US","hi")
        .description("diz ola")
        //.description_localized("en_US","say hello")
        .create_option(|option|{
            option
                .name("nome_do_amigui")
                //.name_localized("en_US","name")
                .description("nome do amigui")
                //.description_localized("en_US","friend's name")
                .kind(CommandOptionType::String)
                .required(true)
        })
}