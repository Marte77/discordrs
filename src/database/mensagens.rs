use serenity::model::prelude::Message;
use serde_json::to_string as to_json_string;
//use serenity::model::application::interaction::{Interaction,InteractionType};
pub struct Mensagem {
    pub mensagem:String,
    pub idutilizador:String,
    pub nomeutilizador:String,
    pub datacriacao:String,
    pub attachments_json:String,
    pub tipo_mensagem:String,
    pub channel_id:String
}

impl Mensagem {
    pub fn new_from_message(message: &Message) -> Self {
        let id = message.author.id.as_u64();
        let mut attachments_str ="[".to_owned();
        for attachment in &message.attachments {
            match to_json_string(&attachment){
                Ok(string) => attachments_str.push_str(&string),
                Err(e) => attachments_str.push_str(&format!("{{\"err\":\"{}\"}}",e))
            }
        }
        let channel_id = message.channel_id.as_u64();
        Self{
            mensagem: message.content.clone(),
            idutilizador: id.to_string(),
            nomeutilizador: message.author.name.clone(),
            datacriacao: message.timestamp.to_string(),
            attachments_json: attachments_str,
            tipo_mensagem:"mensagem normal".to_owned(),
            channel_id:channel_id.to_string()
        }
    }
    /*pub fn new_from_interaction(interaction: &Interaction) -> Option<Self>{
        match interaction.kind(){
            InteractionType::Ping => None,
            InteractionType::ApplicationCommand => {
                //todo
            },
            InteractionType::MessageComponent => {},
            InteractionType::Autocomplete => {},
            InteractionType::ModalSubmit => {},
            InteractionType::Unknown => {}
        }
        
    }   */
}
