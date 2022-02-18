use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::model::channel::Message;
use serenity::model::id::{ChannelId};
use serenity::model::event::ResumedEvent;
use serenity::model::gateway::{Ready,Activity};
use serenity::framework::standard::{
    StandardFramework,
    CommandResult,
};
use serenity::utils::MessageBuilder;
use serde_json;
use std::fs;
use std::collections::HashMap;
use std::process::Command;
use std::str;
use tokio::fs::File;
use songbird::{
    SerenityInit,
    //CoreEvent,
    //Event,
    //EventContext,
    //EventHandler as VoiceEventHandler,
};
use std::cell::RefCell;

thread_local!(static DB_CONNECTION: RefCell<sqlite::Connection> = RefCell::new(sqlite::open("mensagens.db").unwrap()));
const TABELA_DB:&str = "mensagens";
const CREATE_QUERY:&str = "CREATE TABLE if not exists mensagens (mensagem TEXT, idutilizador TEXT, nomeutilizador TEXT)";
fn map(x:usize, from_min:usize, from_max:usize, to_min:usize, to_max:usize) -> usize {
    return (x - from_min) * (to_max - to_min) / (from_max - from_min) + to_min;
}


#[allow(dead_code)]
struct Handler{
    prefixo: String,
    token: String,
    twitter_consumer_key: String,
    twitter_consumer_secret: String,
    twitter_bearer_token: String,
    twitter_access_token_key: String,
    twitter_access_token_secret: String,
    youtube_api_key: String,
    client_idgoogle: String,
    client_secretgoogle: String,
    redirect_urisgoogle: String,
    playing_activity: String,
    activity_tipo: String,
    stream_link: String
}





fn inicializar_handler() -> Handler {
    let pathconfig: String = "./src/config.json".to_owned();
    let json_config_data_string: String = fs::read_to_string(pathconfig).expect("nao consegui ler a config.json"); 
    let json_config: serde_json::Value = serde_json::from_str(& json_config_data_string).expect("nao consegui fazer parse do json");
    return Handler {
        prefixo: json_config["prefix"].to_string().replace("\"",""),
        token:json_config["token"].to_string().replace("\"",""),
        twitter_consumer_key:json_config["twitter_consumer_key"].to_string().replace("\"",""),
        twitter_consumer_secret:json_config["twitter_consumer_secret"].to_string().replace("\"",""),
        twitter_bearer_token:json_config["twitter_bearer_token"].to_string().replace("\"",""),
        twitter_access_token_key:json_config["twitter_access_token_key"].to_string().replace("\"",""),
        twitter_access_token_secret:json_config["twitter_access_token_secret"].to_string().replace("\"",""),
        youtube_api_key:json_config["youtube_api_key"].to_string().replace("\"",""),
        client_idgoogle:json_config["client_idgoogle"].to_string().replace("\"",""),
        client_secretgoogle:json_config["client_secretgoogle"].to_string().replace("\"",""),
        redirect_urisgoogle:json_config["redirect_urisgoogle"].to_string().replace("\"",""),
        playing_activity:json_config["playing_activity"].to_string().replace("\"",""),
        activity_tipo:json_config["activity_tipo"].to_string().replace("\"",""),
        stream_link:json_config["stream_link"].to_string().replace("\"",""),
        
    }
}

async fn msg_log(msg: &Message){
    let mensagem:String = msg.content.clone();
    DB_CONNECTION.with(|cell|{
        let con = cell.borrow_mut();
        con.execute(format!("INSERT INTO {} values ('{}','{}','{}')", TABELA_DB, mensagem, msg.author.id.0, msg.author.name)).ok();
    });
}


//https://docs.rs/serenity/latest/serenity/prelude/trait.EventHandler.html#method.ready
#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message){
        if msg.author.id.0 == 586601655244685318 {
            return;
        }
        msg_log(&msg).await;
        msg_responder(self,&ctx, msg).await;
    }

    async fn resume(&self,_ctx: Context, _: ResumedEvent){
        match &self.activity_tipo as &str {
            "listening" =>_ctx.set_activity(Activity::listening(&self.playing_activity)).await,
            "competing" => _ctx.set_activity(Activity::competing(&self.playing_activity)).await,
            "streaming" => _ctx.set_activity(Activity::streaming(&self.playing_activity,&self.stream_link)).await,
            "watching" => _ctx.set_activity(Activity::watching(&self.playing_activity)).await,
            _ => _ctx.set_activity(Activity::playing(&self.playing_activity)).await
        }
        println!("Estou resumidamente ready");
    }
    async fn ready(&self, _ctx: Context, _data_about_bot: Ready) {
        match &self.activity_tipo as &str {
            "listening" =>_ctx.set_activity(Activity::listening(&self.playing_activity)).await,
            "competing" => _ctx.set_activity(Activity::competing(&self.playing_activity)).await,
            "streaming" => _ctx.set_activity(Activity::streaming(&self.playing_activity,&self.stream_link)).await,
            "watching" => _ctx.set_activity(Activity::watching(&self.playing_activity)).await,
            _ => _ctx.set_activity(Activity::playing(&self.playing_activity)).await
        }
        println!("Estou ready");
    }
}

async fn msg_responder(_handler: &Handler,_ctx: &Context, _msg: Message) {
    if _msg.content.starts_with(_handler.prefixo.as_str()) {
        let conteudomensagem:  Vec<&str> = _msg.content.split(_handler.prefixo.as_str()).collect();
        let mut mensagem: String = String::from("");
        for s in conteudomensagem {
            mensagem.push_str(s);
        }
        if mensagem.chars().next().unwrap() == ' ' {
            mensagem = mensagem[1..mensagem.len()].to_string();
        }
        if mensagem.contains("help") {
            help_msg(_handler, _ctx, _msg).await;
            return;
        }
        if mensagem.contains("avatar") || mensagem.contains("pfp") {
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
        if mensagem.contains("ping") {
            let mut npings = 15;
            for s in mensagem.split_whitespace(){
                let number = match  s.parse::<u32>(){
                    Ok(number) => number,
                    Err(_e) => 15,
                };
                npings = number;
            }   
            spam_ping(_handler, _ctx, _msg,npings).await;
            return;
        }
        if mensagem.contains("xiu") {
            ghost_ping(_handler,_ctx,_msg).await;
            return;
        }
        if mensagem.starts_with("tweet") {
            let mut tweet = mensagem.split_once("tweet").unwrap().1.to_string();
            if tweet.as_str().len() == 0 {
                if _msg.attachments.len() == 0 {
                    _msg.reply(_ctx, "mete texto do tweet").await.ok();
                    return;
                }else {
                    tweet = "".to_owned();
                }
            }
            make_tweet(tweet, _handler, _ctx, _msg).await;
            return;
        }
        if mensagem.starts_with("join"){
            match join_call(&_handler,_ctx, &_msg).await {
                Ok(_)=>{},
                Err(_)=>{},
            };
        }

    }else{
        let mensagem = _msg.content.clone();
        if mensagem.contains("help") {
            match _msg.mentions_me(&_ctx.http).await{
                Ok(res) => if res {
                    help_msg(_handler,_ctx,_msg).await;
                    return;},
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
                None => {_msg.reply(_ctx, "ocorreu um erro").await.ok(); return;},
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
            ghost_ping(_handler,_ctx,_msg).await;
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
}

async fn join_call(_handler: &Handler,ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    let guild_id = guild.id;

    let channel_id = guild
        .voice_states.get(&msg.author.id)
        .and_then(|voice_state| voice_state.channel_id);

    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            msg.reply(ctx, "Not in a voice channel").await.ok();

            return Ok(());
        }
    };

    let manager = songbird::serenity::get(ctx).await
        .expect("Songbird Voice client placed in at initialisation.").clone();

    let _handler = manager.join(guild_id, connect_to).await;
    Ok(())
}


async fn make_tweet(tweet:String,_handler: &Handler,_ctx: &Context, _msg: Message){
    let con_token = egg_mode::KeyPair::new(_handler.twitter_consumer_key.clone(), _handler.twitter_consumer_secret.clone());
    let access_token = egg_mode::KeyPair::new(_handler.twitter_access_token_key.clone(), _handler.twitter_access_token_secret.clone());
    let token = egg_mode::Token::Access {
        consumer: con_token,
        access: access_token,
    };
    let mut tweetbuilder = egg_mode::tweet::DraftTweet::new(tweet);
    if _msg.attachments.len() > 0 {
        for attachment in &_msg.attachments {
            let tipooption = _msg.attachments[0].url.split(".").last();
            let tipoimg = tipooption.as_deref().unwrap_or("default");
            if tipoimg.contains("png") || tipoimg.contains("gif") || tipoimg.contains("mp4") || tipoimg.contains("jpg") || tipoimg.contains("jpeg") || tipoimg.contains("webp") {
                let bytes = match attachment.download().await {
                    Ok(b) => b,
                    Err(_err) => {println!("{:#?}",_err);break},
                };
                let mediatype = match tipoimg {
                    "png" => egg_mode::media::media_types::image_png(),
                    "jpg" => egg_mode::media::media_types::image_jpg(),
                    "jpeg" => egg_mode::media::media_types::image_jpg(),
                    "mp4" => egg_mode::media::media_types::video_mp4(),
                    "gif" => egg_mode::media::media_types::image_gif(),
                    "webp" => egg_mode::media::media_types::image_webp(),
                    _ => egg_mode::media::media_types::image_png()
                };
                match egg_mode::media::upload_media(&bytes, &mediatype, &token).await {
                    Ok(media) => {
                        let wait_time = std::time::Duration::from_millis(300);
                        for _i in 0..15 {
                            let id = media.id.clone();
                            match egg_mode::media::get_status(id, &token).await {
                                Ok(_status) => {
                                    if _status.is_valid() {
                                        tweetbuilder.add_media(_status.id);
                                        break;
                                    }else{
                                        std::thread::sleep(wait_time);
                                    }
                                },
                                Err(_) =>{
                                    std::thread::sleep(wait_time);
                                }
                            };
                            
                        }
                    },
                    Err(_e) => {
                    },
                };
            }     
        }
    }
    match tweetbuilder.send(&token).await{
        Ok(twee) => _msg.reply(_ctx, format!("Tweet aqui https://twitter.com/IlikeVeryPeidos/status/{}",twee.id)).await.ok(),
        Err(_e) => {println!("{:#?}",_e);_msg.reply(_ctx, "erro a fazer o tweet").await.ok()},
    };
    

}

async fn help_msg(_handler: &Handler,_ctx: &Context, _msg: Message){
    _msg.reply(_ctx, r#"
        `help` -> isto;
        `avatar` \|| `pfp` -> link com a imagem;
        `ping @user <npings>` -> pingar utilizador @user com 15 pings ou definir <npings>;
        `tweet <conteudo>` -> tweetar na minha conta
    "#).await.ok();
}



async fn ghost_ping(_handler: &Handler,_ctx: &Context, _msg: Message){
    _msg.reply(_ctx, "ok mano desculpa :pensive:").await.ok();
    let guildid = _msg.guild_id.unwrap();
    let channels = match guildid.channels(&_ctx.http).await {
        Ok(v) => v,
        Err(_e) => HashMap::new()
    };
    if channels.len() <= 0{
        return;
    }
    
    
    for _i in 0..15 {
        let randomvarout =match Command::new("/usr/bin/bash").args(&["-c", "echo $RANDOM"])
        .output(){
            Ok(res) => res,
            Err(_e) => {break},
        };
        let randomvarstr = match str::from_utf8(&randomvarout.stdout){
            Ok(res1) => res1,
            Err(_e) => "0",
        };
        let mut randomvar = randomvarstr.replace("\n", "");
        randomvar = randomvar.replace("\"", "");
        let random = match randomvar.parse::<usize>(){
            Ok(n) => n,
            Err(_e) => 0,
        };
        let rand = map(random ,0,32767,0,channels.len()-1);
        let channelid = channels.keys().nth(rand).unwrap_or(&ChannelId(802241972168687647)); //get random key aka random channelid
        let msgbuild = MessageBuilder::new().mention(&_msg.author).build();
        let msg = channelid.say(&_ctx.http,msgbuild).await;
        match msg {
            Ok(m) => {channelid.delete_message(&_ctx.http,m.id).await.ok();},
            Err(_e) =>{ },
        }
        
    }
}

async fn spam_ping(_handler: &Handler,_ctx: &Context, _msg: Message, npings: u32){
    if _msg.mentions.len() > 1 {
        _msg.reply_ping(_ctx, "Apenas um utilizador pode ser pingado").await.ok();
        return;
    }
    if _msg.mentions.len() == 0 {
        _msg.reply_ping(_ctx, "pinga alguem, atrasado").await.ok();
        return;
    }
    let channelid = ChannelId(_msg.channel_id.0);
    let usr = &_msg.mentions[0];
    let message = MessageBuilder::new()
        .push_bold("boas")
        .mention(usr)
        .build(); 
    
    for _i in 0..npings{
        channelid.say(&_ctx.http,message.clone()).await.ok();
    }
}


#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix(inicializar_handler().prefixo)) // set the bot's prefix to "~"
        ;
    DB_CONNECTION.with(|odb_cell| {
        let odb = odb_cell.borrow_mut();
        match odb.execute(CREATE_QUERY) {
            Ok(_) => {println!("sucesso");},
            Err(e) => { println!("erro a criar a bd {:#?}",e);}
        };
        // code that uses odb goes here
    });
    // Login with a bot token from the environment
    let token = inicializar_handler().token;
    let mut client = Client::builder(token)
        .event_handler(inicializar_handler())
        .framework(framework)
        .register_songbird()
        .await
        .expect("Error creating client");
    
    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}
