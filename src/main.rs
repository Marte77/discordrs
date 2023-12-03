use crate::database::{queries};
pub mod database;

use crate::non_command_input::common::{msg_responder};
pub mod non_command_input;

pub mod helper;

mod commands;
use crate::commands::twitter::*;
use crate::commands::common::*;
use crate::commands::slash;

use crate::handler::handler::Handler;
pub mod handler;

use std::collections::HashSet;
use std::sync::Arc;

use handler::handler::HandlerStrings;
use serenity::async_trait;
use serenity::client::bridge::gateway::ShardManager;
use serenity::prelude::*;
use serenity::model::gateway::{Ready,Activity};
use serenity::model::event::ResumedEvent;
use serenity::model::channel::Message;
use serenity::model::application::command::Command;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::http::Http;
use serenity::framework::standard::StandardFramework;
use serenity::framework::standard::macros::group;

use tracing::error;

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

#[async_trait]
impl EventHandler for Handler {
    async fn resume(&self,_ctx: Context, _: ResumedEvent){
        match &self.handler_strings.activity_tipo as &str {
            "listening" =>_ctx.set_activity(Activity::listening(&self.handler_strings.playing_activity)).await,
            "competing" => _ctx.set_activity(Activity::competing(&self.handler_strings.playing_activity)).await,
            "streaming" => _ctx.set_activity(Activity::streaming(&self.handler_strings.playing_activity,&self.handler_strings.stream_link)).await,
            "watching" => _ctx.set_activity(Activity::watching(&self.handler_strings.playing_activity)).await,
            _ => _ctx.set_activity(Activity::playing(&self.handler_strings.playing_activity)).await
        }
        println!("Estou resumidamente ready");
    }
    async fn ready(&self, _ctx: Context, _data_about_bot: Ready) {
        //registar activity type
        match &self.handler_strings.activity_tipo as &str {
            "listening" =>_ctx.set_activity(Activity::listening(&self.handler_strings.playing_activity)).await,
            "competing" => _ctx.set_activity(Activity::competing(&self.handler_strings.playing_activity)).await,
            "streaming" => _ctx.set_activity(Activity::streaming(&self.handler_strings.playing_activity,&self.handler_strings.stream_link)).await,
            "watching" => _ctx.set_activity(Activity::watching(&self.handler_strings.playing_activity)).await,
            _ => _ctx.set_activity(Activity::playing(&self.handler_strings.playing_activity)).await
        }

        //registar slash commands
        match Command::create_global_application_command(&_ctx.http, |command|{
            slash::ola::register(command)
        }).await{
            Ok(_) => {},
            Err(e) =>{
                println!("erro a registar os comandos, {}", e);
            }
        };

        println!("Estou ready");
    }
    async fn message(&self,_ctx: Context, msg: Message){
        queries::log_message(&msg,&self).await.ok();
        if msg.author.id.0 == 586601655244685318 {
            return;
        }
        msg_responder(&_ctx, msg).await;
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            //queries::insert_interaction(&interaction,&self);
            let content = match command.data.name.as_str() {
                "ola" => slash::ola::run(&command.data.options),
                _ => "not implemented :(".to_string(),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }
}

#[group]
#[commands(help,oas,avatar,ping,xiu,tweet,tweedown, wakeonlan)]
struct General;

#[tokio::main]
async fn main(){
    let handler = Handler::new().await;
    tracing_subscriber::fmt::init();
    let http = Http::new(&handler.handler_strings.token);

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
        .configure(|c| c.owners(owners).prefix(&handler.handler_strings.prefixo)).group(&GENERAL_GROUP) // set the bot's prefix to "~"
        ;
    let intents = serenity::model::gateway::GatewayIntents::non_privileged() 
        | serenity::model::gateway::GatewayIntents::DIRECT_MESSAGES
        | serenity::model::gateway::GatewayIntents::GUILD_MESSAGES
        | serenity::model::gateway::GatewayIntents::MESSAGE_CONTENT
        ;
    let handler_strings = (&handler).handler_strings.clone();
    let mut client = Client::builder(&handler_strings.token, intents)
        .framework(framework)
        .event_handler(handler)
        .await
        .expect("Error creating client");
    
    {
        let mut data = client.data.write().await;
        data.insert::<HandlerStrings>(handler_strings);
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