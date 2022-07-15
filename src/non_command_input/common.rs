use serenity::prelude::*;
use serenity::model::channel::Message;
use tokio::fs::File;
use std::collections::HashMap;
use std::process::Command;
use std::str;
use serenity::utils::MessageBuilder;
use serenity::model::id::{ChannelId};

fn map(x:usize, from_min:usize, from_max:usize, to_min:usize, to_max:usize) -> usize {
    return (x - from_min) * (to_max - to_min) / (from_max - from_min) + to_min;
}


pub async fn msg_responder(_ctx: &Context, _msg: Message) {
    let mensagem = _msg.content.clone();
        if mensagem.contains("help") {
            match _msg.mentions_me(&_ctx.http).await{
                Ok(res) => if res {
                    _msg.reply(_ctx, r#"
                    `help` -> isto;
                    `avatar` \|| `pfp` -> link com a imagem;
                    `ping @user <npings>` -> pingar utilizador @user com 15 pings ou definir <npings>;
                    `tweet <conteudo>` -> tweetar na minha conta
                    `tweedown <link do video>` -> fazer download dum video do twitter
                    "#).await.ok();
                    return;
                },
                Err(_) => {}
            };
        }
        if mensagem == "gg" {
            _msg.reply(_ctx, "gg").await.ok();
            return;
        }
        if mensagem.to_ascii_lowercase().contains("pog") {
            let gid = match _msg.guild_id{
                Some(id) => id,
                None => {return;},
            };

            let emojis = match gid.emojis(&_ctx.http).await {
                Err(_e) => Vec::new(),
                Ok(emojis) => emojis,
            };
            for emoji in emojis {
                if emoji.name == "poggaroo" || emoji.name == "pogt" {
                    _msg.react(_ctx, emoji).await.ok();
                }
            }
            
        }
        if mensagem.contains("lets go") ||  mensagem.contains("let's go") {
            _msg.reply(_ctx,"https://64.media.tumblr.com/cdff5ebf86bd4027c164ea911ff12c38/68d632cb07902a7f-b5/s400x600/2d2d7b1d6559a4e1890138c7952f08a84774502e.png")
            .await.ok();
            return;
        }  
        if mensagem.contains("sus") ||  mensagem.contains("impostor") ||  mensagem.contains("among us") {
            _msg.reply(_ctx,"https://cdn.discordapp.com/attachments/556495723336564744/794658710907125820/mqdefault.png")
            .await.ok();
            return;
        }  
        if mensagem.contains("xiu bot") || (mensagem.contains("xiu") && _msg.mentions_me(&_ctx.http).await.unwrap()){
            xiu(_ctx,_msg).await;
            return;
        }
        if mensagem.contains("boas pessoal voces sabem quem fala") || mensagem.contains("boas pessoal vocês sabem quem fala") {
            _msg.reply(_ctx, "Daqui é o Tiagovski a rebentar a escala\nMinecraft eu gosto de jogar!\nCom os meus episódios vos animar!\nÁters seus filhos da puta\nVocês devem ter memória curta\nNão se lembram de eu vos dizer?\nIdes pro caralho vão se fuder!\nAntes gostavam de mim mas agora não\nInveja é lixada pois é irmão\nOs meus subs tão no coração\nOs Áters eu desfaço com a mão\nMinecraft eu sei jogar\nMas vocês nem isso sabem apreciar\nCritiquem á vontade dêem a opinião\nMas dar dislike é a vossa profissão\nSó fazem isso para ter reputação\nQuando fazem videos daquele lucifrão\nMas agora falo dos meus amores\nQue são os meus subscritores\nVocês conhecem os meus parceiros\nAqueles gajos mesmo porreiros\nTemos a agda sempre a dizer\nPor favor Tiago não quero morrer!\nDepois vem a kika a falar\nO problema e que ela não se sabe calar\nO clipe é o porco bem maroto\nMas ele vale bem mais que um escroto\nO LegendBoy é um rico selo\nMas só porque ele tem cabelo\nA musica foi pequena mas de bom agrado\nAgora vou gravar fica ai agarrado!\nVá pessoal fiquem bem\nPorque sou eu quem vos entretém\nVá, vou bazar\nPorque esta me a apetecer jogar!\nFUI").await.ok();
            return;
        }
        if mensagem.contains("😭") || mensagem == ";(" || mensagem.contains("vou chorar"){
            match File::open("./src/gonnacry.mp4").await {
                Ok(file) => {
                    let files = vec![(&file,"gonnacry.mp4")];
                    _msg.channel_id.send_files(&_ctx.http,files, |m| m.content("")).await.ok();
                    return;
                },
                Err(_e) => {}
            };
            
        }
}

pub async fn xiu(ctx: &Context, msg: Message) {
    msg.reply(ctx, "ok mano desculpa :pensive:").await.ok();
    let guildid = msg.guild_id.unwrap();
    let channels = match guildid.channels(&ctx).await {
        Ok(v) => v,
        Err(_e) => HashMap::new()
    };
    if channels.len() <= 0{
        return;
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
}