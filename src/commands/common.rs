use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::*;
use serenity::model::user::User;
use serenity::model::id::{UserId,ChannelId};
use serenity::utils::MessageBuilder;

use std::collections::HashMap;
use std::process::Command;
use std::str;

use crate::handler::handler::HandlerStrings;

fn map(x:usize, from_min:usize, from_max:usize, to_min:usize, to_max:usize) -> usize {
    return (x - from_min) * (to_max - to_min) / (from_max - from_min) + to_min;
}

#[command]
#[aliases("ajuda")]
pub async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    let rwlock = ctx.data.read().await;
    let handler = rwlock.get::<HandlerStrings>();
    msg.reply(ctx, format!(r#"
    para usar os comandos, usar o prefixo `{}`
`help` -> isto;
`avatar` \|| `pfp` -> link com a imagem;
`ping @user <npings>` -> pingar utilizador @user com 15 pings ou definir <npings>;
`tweet <conteudo>` -> tweetar na minha conta
`tweedown <link do video>` -> fazer download dum video do twitter
`wakeonlan [wifi]` -> enviar packet wakeonlan para o pc com servidor do mine
"#, if handler.is_some() { handler.unwrap().prefixo.as_str() } else { "erro a obter prefixo" })).await?;
    Ok(())
}

#[command]
pub async fn oas(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "chamaste?").await?;
    Ok(())
}

#[command]
#[aliases("pfp","foto")]
pub async fn avatar(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    if let Ok(userid) = args.single::<UserId>() {
        let user = userid.to_user(ctx).await?;
        msg.reply(ctx, user.avatar_url().unwrap().as_str()).await?;
    }else {
        msg.reply(ctx, msg.author.avatar_url().unwrap().as_str()).await?;
    }
    Ok(())
}

#[command]
pub async fn ping(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let user: User = args.single::<UserId>()?.to_user(ctx).await?;
    let npings: i64 = args.single::<i64>()?;
    let channelid = ChannelId(msg.channel_id.0);
    let message = MessageBuilder::new()
        .push_bold("boas")
        .mention(&user)
        .build(); 


    for _i in 0..npings{
        channelid.say(&ctx.http,message.clone()).await.ok();
    }   
    Ok(())
}

#[command]
pub async fn xiu(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "ok mano desculpa :pensive:").await.ok();
    let guildid = msg.guild_id.unwrap();
    let channels = match guildid.channels(&ctx).await {
        Ok(v) => v,
        Err(_e) => HashMap::new()
    };
    if channels.len() <= 0{
        return Ok(());
    }
    let shellpath = match std::env::consts::OS {
        "macos" => "/bin/zsh",
        _ => "/usr/bin/bash"
    };
    for _i in 0..15 {
        let randomvarout =match Command::new(shellpath).args(&["-c", "echo $RANDOM"])
        .output(){
            Ok(res) => res,
            Err(_e) => {println!("{:#?}",_e);break},
        };
        let randomvarstr = match str::from_utf8(&randomvarout.stdout){
            Ok(res1) => res1,
            Err(_e) => "0",
        };
        let randomvar = randomvarstr.replace("\n", "").replace("\"", "");
        let random = match randomvar.parse::<usize>(){
            Ok(n) => n,
            Err(_e) => 0,
        };
        let rand = map(random ,0,32767,0,channels.len()-1);
        let channelid = channels.keys().nth(rand).unwrap_or(&ChannelId(802241972168687647)); //get random key aka random channelid
        let msgbuild = MessageBuilder::new().mention(&msg.author).build();
        let msg = channelid.say(&ctx,msgbuild).await;
        match msg {
            Ok(m) => {channelid.delete_message(&ctx.http,m.id).await.ok();},
            Err(_e) =>{ },
        }
        
    }
    Ok(())
}

#[command]
#[aliases("wol")]
pub async fn wakeonlan(ctx: &Context, msg: &Message) -> CommandResult {
    ctx.dnd().await;
    let reply_result = msg.reply(ctx, "espera um coche").await;
    let wol_packet = if !msg.content.contains("wifi") 
    {wakey::WolPacket::from_string("98:28:a6:2f:a0:ec",':')}
    else {wakey::WolPacket::from_string("f8:a2:d6:50:2f:75",':')};
    #[allow(unused_assignments)]
    let mut edit_builder = "".to_owned();
    match wol_packet {
        Ok(wol) => {
            match wol.send_magic() {
                Ok(_) => edit_builder="wol packet enviado".to_owned(),
                Err(magic_error) => edit_builder=format!("erro a enviar wol packet {:#?}", magic_error)
            };
        },
        Err(wol_error) =>{
            edit_builder=format!("erro a criar wol packet {:#?}", wol_error);
        }
    }
    ctx.online().await;
    if let Ok(mut reply) = reply_result {
        reply.edit(ctx, |x|{
            x.content(edit_builder)
        }).await?;
    }
    Ok(())
}