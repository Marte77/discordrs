#![allow(unused_imports)]
use serenity::framework::standard::macros::{command};
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::*;
use serenity::model::user::User;
use serenity::model::id::{UserId,ChannelId};
use serenity::utils::MessageBuilder;
use std::collections::HashMap;
use std::process::Command;
use serenity::model::channel::Attachment;
use std::str;
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
    let conteudo_tweet: String = _args.raw().collect::<Vec<&str>>().join(" ");
    if conteudo_tweet.len() == 0 {
        msg.reply(ctx, "mete texto do tweet maninho").await?;
        return Ok(());
    }
    make_tweet(conteudo_tweet, ctx, &msg).await?;
    Ok(())
}