use crate::helper_structs;
use serenity::client::{Context};
use serenity::model::channel::Message;
use serenity::utils::MessageBuilder;
pub async fn get_pfp_discord(_handler: &helper_structs::Handler,_ctx: &Context, _msg: Message){
    if _msg.mentions.len() == 0 {
        _msg.reply(_ctx, _msg.author.avatar_url().unwrap().as_str()).await.ok();
        return;
    }
    let mut message = MessageBuilder::new();
    for user in &_msg.mentions {
        let mut c: String = "Pfp de ".to_owned();
        c.push_str(user.name.as_str());
        c.push_str(": ");
        c.push_str(user.avatar_url().unwrap().as_str());
        message.push_line(c);
    }
    let msgfinal = message.build();
    _msg.reply(_ctx, msgfinal.as_str()).await.ok();
}