use crate::helper_structs;
#[allow(unused_imports)]
use crate::helper_funcs;
use serenity::client::{Context};
use serenity::model::channel::Message;
use egg_mode::Response;
use egg_mode::tweet::Tweet;
use serenity::utils::MessageBuilder;
use std::io::Write; // bring trait into scope
use std::fs;

pub fn create_auth_token(_handler: &helper_structs::Handler) -> egg_mode::Token{
    let con_token = egg_mode::KeyPair::new(_handler.twitter_consumer_key.clone(), _handler.twitter_consumer_secret.clone());
    let access_token = egg_mode::KeyPair::new(_handler.twitter_access_token_key.clone(), _handler.twitter_access_token_secret.clone());
    return egg_mode::Token::Access {
        consumer: con_token,
        access: access_token,
    };
}

pub async fn make_tweet(tweet:String,_handler: &helper_structs::Handler,_ctx: &Context, _msg: Message){
    let token = create_auth_token(_handler);
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

pub async fn download_video_twitter(_handler: &helper_structs::Handler,_ctx: &Context, _msg: Message, mensagem:String){
    let tweetstr:String = mensagem.split_once("twt ").unwrap().1.to_string();
    //println!("{}",helper_funcs::extract_url(tweet));
    let split:Vec<&str> = tweetstr.split("/").collect();
    let straux1 = split[5].to_owned();
    let straux:Vec<&str> = straux1.split("?").collect();
    let id:u64 = straux[0].parse::<u64>().unwrap();
    let tweetlookup = egg_mode::tweet::show(id, &create_auth_token(_handler)).await;
    if tweetlookup.is_ok() {
        let res1:Response<Tweet> = tweetlookup.unwrap();
        let tweet: Tweet = res1.response;
        let mut msgfinal = MessageBuilder::new();
        
        match tweet.extended_entities {
            Some(entities) => {
                let media: Vec<egg_mode::entities::MediaEntity> = entities.media;
                let mut entrou: bool = false;
                let mut videoFinal: Option<egg_mode::entities::VideoVariant> = None;
                for m in media {
                    if m.media_type == egg_mode::entities::MediaType::Video {
                        let videoInfo = m.video_info.unwrap();
                        let mut deubreak = false;
                        for video in videoInfo.variants {
                            //msgfinal.push_line(video.url);
                            if video.content_type.type_().as_str().eq("video") {
                                videoFinal = Some(video);
                                deubreak = true;
                                break;
                            }
                        }
                        if deubreak {
                            entrou = true;
                            break;
                        }
                    }
                }
                if !entrou {
                    msgfinal.push("o tweet nao tem video");
                } else {
                    let video = std::fs::File::create("temp.mp4");
                    if video.is_ok() {
                        match reqwest::blocking::get(videoFinal.unwrap().url) {
                            Ok(mut file) => {
                                let mut buf: Vec<u8> = vec![];
                                match file.copy_to(&mut buf){
                                    Ok(_)=>{
                                        match video.unwrap().write_all(&mut buf){
                                            Ok(_)=>{
                                                //TODO ENVIAR FICHEIRO
                                            },
                                            Err(_)=>{ msgfinal.push_line("erro a fazer download do video");}
                                        };
                                    },
                                    Err(_)=>{
                                        msgfinal.push_line("erro a fazer download do video");
                                    }
                                };
                            },
                            Err(_) => {
                                msgfinal.push_line("erro a fazer download do video");
                            }
                        };
                    }
                }
            },
            None => {
                msgfinal.push("O tweet nao tem media nenhum");
            },
        }
        _msg.reply(_ctx,msgfinal.build().as_str()).await.ok();
    } else {
        _msg.reply(_ctx,"erro").await.ok();
    }
}
