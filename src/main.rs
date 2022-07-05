mod commands;
use crate::commands::twitter::*;
use crate::commands::common::*;
mod non_command_input;
use crate::non_command_input::common::*;
use std::env;
use std::collections::HashSet;
use std::sync::Arc;

use serenity::async_trait;
use serenity::client::bridge::gateway::ShardManager;
use serenity::prelude::*;
use serenity::model::gateway::{Ready,Activity};
use serenity::model::event::ResumedEvent;
use serenity::model::channel::Message;
use serenity::http::Http;
use serenity::framework::standard::StandardFramework;
use serenity::framework::standard::macros::{group};
use tracing::{error};

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
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

impl Handler {
    fn new() -> Self {
        dotenv::dotenv().expect("Não consegui carregar o .env");
        Self{
            prefixo:env::var("PREFIX").expect("Expected a PREFIX field in the environment"),
            token:env::var("TOKEN").expect("Expected a TOKEN (discord token) field in the environment"),
            twitter_consumer_key:env::var("TWITTER_CONSUMER_KEY").expect("Expected a TWITTER_CONSUMER_KEY field in the environment"),
            twitter_consumer_secret:env::var("TWITTER_CONSUMER_SECRET").expect("Expected a TWITTER_CONSUMER_SECRET field in the environment"),
            twitter_bearer_token:env::var("TWITTER_BEARER_TOKEN").expect("Expected a TWITTER_BEARER_TOKEN field in the environment"),
            twitter_access_token_key:env::var("TWITTER_ACCESS_TOKEN_KEY").expect("Expected a TWITTER_ACCESS_TOKEN_KEY field in the environment"),
            twitter_access_token_secret:env::var("TWITTER_ACCESS_TOKEN_SECRET").expect("Expected a TWITTER_ACCESS_TOKEN_SECRET field in the environment"),
            youtube_api_key:env::var("YOUTUBE_API_KEY").expect("Expected a YOUTUBE_API_KEY field in the environment"),
            client_idgoogle:env::var("CLIENT_IDGOOGLE").expect("Expected a CLIENT_IDGOOGLE field in the environment"),
            client_secretgoogle:env::var("CLIENT_SECRETGOOGLE").expect("Expected a CLIENT_SECRETGOOGLE field in the environment"),
            redirect_urisgoogle:env::var("REDIRECT_URISGOOGLE").expect("Expected a REDIRECT_URISGOOGLE field in the environment"),
            playing_activity:env::var("PLAYING_ACTIVITY").expect("Expected a PLAYING_ACTIVITY field in the environment"),
            activity_tipo:env::var("ACTIVITY_TIPO").expect("Expected a ACTIVITY_TIPO field in the environment"),
            stream_link:env::var("STREAM_LINK").expect("Expected a STREAM_LINK field in the environment"),
        }
    }
}

#[async_trait]
impl EventHandler for Handler {
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
    async fn message(&self,_ctx: Context, msg: Message){
        if msg.author.id.0 == 586601655244685318 {
            return;
        }
        //msg_log(&msg).await;
        msg_responder(&_ctx, msg).await;
    }
}

#[group]
#[commands(help,oas,avatar,ping,xiu,tweet)]
struct General;

#[tokio::main]
async fn main(){
    let handler = Handler::new();
    tracing_subscriber::fmt::init();
    let http = Http::new(&handler.token);

    // We will fetch your bot's owners and id
    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        },
        Err(why) => panic!("Could not access application info: {:?}", why),
    };
    
    let framework = StandardFramework::new()
        .configure(|c| c.owners(owners).prefix(&handler.prefixo)).group(&GENERAL_GROUP) // set the bot's prefix to "~"
        ;
    let intents = serenity::model::gateway::GatewayIntents::non_privileged() 
        | serenity::model::gateway::GatewayIntents::DIRECT_MESSAGES
        | serenity::model::gateway::GatewayIntents::GUILD_MESSAGES
        | serenity::model::gateway::GatewayIntents::MESSAGE_CONTENT
        ;
    let mut client = Client::builder(&handler.token, intents)
        .framework(framework)
        .event_handler(handler)
        .await
        .expect("Error creating client");
    
    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
    }

    let shard_manager = client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.expect("Could not register ctrl+c handler");
        shard_manager.lock().await.shutdown_all().await;
    });
    
    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}