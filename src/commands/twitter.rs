use serenity::framework::standard::macros::{command};
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::*;
use std::env;
use tokio::time::sleep;

async fn make_tweet(tweet:String,_ctx: &Context, _msg: &Message) -> CommandResult {
    let twitter_consumer_key = env::var("TWITTER_CONSUMER_KEY").expect("Expected a TWITTER_CONSUMER_KEY field in the environment");
    let twitter_consumer_secret = env::var("TWITTER_CONSUMER_SECRET").expect("Expected a TWITTER_CONSUMER_SECRET field in the environment");
    let twitter_access_token_key = env::var("TWITTER_ACCESS_TOKEN_KEY").expect("Expected a TWITTER_ACCESS_TOKEN_KEY field in the environment");
    let twitter_access_token_secret = env::var("TWITTER_ACCESS_TOKEN_SECRET").expect("Expected a TWITTER_ACCESS_TOKEN_SECRET field in the environment");

    let con_token = egg_mode::KeyPair::new(twitter_consumer_key, twitter_consumer_secret);
    let access_token = egg_mode::KeyPair::new(twitter_access_token_key, twitter_access_token_secret);
    let token = egg_mode::Token::Access {
        consumer: con_token,
        access: access_token,
    };
    let mut tweetbuilder = egg_mode::tweet::DraftTweet::new(tweet);
    if _msg.attachments.len() > 0 {
        for attachment in &_msg.attachments {
            let tipooption = attachment.url.split(".").last();
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
                if let Ok(media) = egg_mode::media::upload_media(&bytes.as_slice(), &mediatype, &token).await {
                    if media.is_valid() {
                        match egg_mode::media::get_status(media.id.clone(), &token).await {
                            Ok(_status) => {
                                tweetbuilder.add_media(media.id);
                            },
                            Err(_) =>{
                            }
                        };
                    }
                }
            } 
        }
    }
    let mut nao_deu_erro = true;
    let mut message: Message = _msg.reply(_ctx,"A processar tweet").await?;
    for handle in &tweetbuilder.media_ids {
        for ct in 0..=60u32 {
            match egg_mode::media::get_status(handle.clone(), &token).await?.progress {
                None | Some(egg_mode::media::ProgressInfo::Success) => {
                    break;
                }
                Some(egg_mode::media::ProgressInfo::Pending(_)) | Some(egg_mode::media::ProgressInfo::InProgress(_)) => {
                    sleep(std::time::Duration::from_secs(1)).await;
                }
                Some(egg_mode::media::ProgressInfo::Failed(err)) => Err(err)?,
            }
            if ct == 60 {
                nao_deu_erro = false;
            }
        }
    }
    if nao_deu_erro {
        match tweetbuilder.send(&token).await{
            Ok(twee) => message.edit(_ctx, |m| m.content(format!("Tweet aqui https://twitter.com/IlikeVeryPeidos/status/{}",twee.id))).await.ok(),
            Err(_e) => {println!("{:#?}",_e);message.edit(_ctx, |m| m.content("erro a fazer o tweet")).await.ok()},
        };
    }else{
        message.edit(_ctx, |m| m.content("erro a fazer upload")).await.ok();
    }
    Ok(())

}


#[command]
pub async fn tweet(ctx: &Context, msg: &Message, mut _args: Args) -> CommandResult {
    match env::var("CAN_TWEET") {
        Ok(val) => {
            match val.as_str() {
                "true" => (),
                "false" => {
                    msg.reply(ctx, "Tweetar não está ativo").await?;
                    return Ok(());
                }
                _ => {
                    msg.reply(ctx, "env variable CAN_TWEET with wrong value").await?;
                    return Ok(());
                }
            }
        },
        Err(_)=>{
            msg.reply(ctx, "env variable CAN_TWEET not found").await?;
            return Ok(());
        }
    }
    
    let conteudo_tweet: String = _args.raw().collect::<Vec<&str>>().join(" ");
    if conteudo_tweet.len() == 0 {
        msg.reply(ctx, "mete texto do tweet maninho").await?;
        return Ok(());
    }
    make_tweet(conteudo_tweet, ctx, &msg).await?;
    Ok(())
}

#[command]
pub async fn tweedown(ctx: &Context, msg: &Message, mut _args: Args) -> CommandResult {
    let twitter_consumer_key = env::var("TWITTER_CONSUMER_KEY").expect("Expected a TWITTER_CONSUMER_KEY field in the environment");
    let twitter_consumer_secret = env::var("TWITTER_CONSUMER_SECRET").expect("Expected a TWITTER_CONSUMER_SECRET field in the environment");
    let twitter_access_token_key = env::var("TWITTER_ACCESS_TOKEN_KEY").expect("Expected a TWITTER_ACCESS_TOKEN_KEY field in the environment");
    let twitter_access_token_secret = env::var("TWITTER_ACCESS_TOKEN_SECRET").expect("Expected a TWITTER_ACCESS_TOKEN_SECRET field in the environment");

    let con_token = egg_mode::KeyPair::new(twitter_consumer_key, twitter_consumer_secret);
    let access_token = egg_mode::KeyPair::new(twitter_access_token_key, twitter_access_token_secret);
    let token = egg_mode::Token::Access {
        consumer: con_token,
        access: access_token,
    };
    

    let typing = msg.channel_id.start_typing(&ctx.http)?;
    match _args.single::<String>() {
        Ok(l) => {
            let mut url: Vec<&str> = l.split("?").collect();
            url = url[0].split("/").collect();
            if url.len() < 6 || url[5] == ""{
                msg.reply(ctx, "Link inválido").await?;
                let _ = typing.stop();
                return Ok(())
            }
            let tweetid: u64 = match url[5].parse::<u64>() {
                Ok(id) => id,
                Err(_) => {
                    msg.reply(ctx, "id inválido").await?;
                    let _ = typing.stop();
                    return Ok(())
                }
            };
            let tweet: Option<egg_mode::tweet::ExtendedTweetEntities> = match egg_mode::tweet::show(tweetid, &token).await {
                Ok(tw) => tw.response.extended_entities,
                Err(_) => {
                    msg.reply(ctx, "erro a encontrar o tweet").await?;
                    let _ = typing.stop();
                    return Ok(())
                }
            };
            match tweet {
                Some(media) => {
                    for m in media.media {
                        if m.media_type == egg_mode::entities::MediaType::Video{
                            let vidinfo = m.video_info;
                            match vidinfo {
                                Some(videoinfo) => {
                                    let video_mais_qualidade: &egg_mode::entities::VideoVariant = match videoinfo.variants.last(){
                                        Some(video) => video,
                                        None =>{
                                            msg.reply(ctx, "Erro").await?;
                                            let _ = typing.stop();
                                            return Ok(())
                                        }
                                    };
                                    let response = reqwest::get(video_mais_qualidade.url.to_owned()).await?.bytes().await?;
                                    let mut fich: std::fs::File = std::fs::File::create("temp.mp4")?;
                                    std::io::copy(&mut response.as_ref(), &mut fich)?;
                                    let files = vec!["temp.mp4"];
                                    msg.channel_id.send_files(ctx,files, |m| m.content("")).await?;
                                    std::fs::remove_file("temp.mp4")?;
                                    let _ = typing.stop();
                                    return Ok(())
                                },
                                None =>{
                                    msg.reply(ctx, "Erro").await?;
                                    let _ = typing.stop();
                                    return Ok(())
                                }
                            }
                        }
                    }
                    let _ = typing.stop();
                    msg.reply(ctx, "Tweet não tem vídeo").await?;
                    return Ok(())
                },
                None => {
                    let _ = typing.stop();
                    msg.reply(ctx, "Tweet não tem vídeo").await?;
                    return Ok(())
                }
            }
        },
        Err(_) => {
            let _ = typing.stop();
            msg.reply(ctx, "Mete link seu burro").await?;
            return Ok(())
        }
    }
}