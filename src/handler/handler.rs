use std::env;
use crate::database::queries;
pub struct Handler{
    pub prefixo: String,
    pub token: String,
    //twitter_consumer_key: String,
    //twitter_consumer_secret: String,
    //twitter_bearer_token: String,
    //twitter_access_token_key: String,
    //twitter_access_token_secret: String,
    //youtube_api_key: String,
    //client_idgoogle: String,
    //client_secretgoogle: String,
    //redirect_urisgoogle: String,
    pub playing_activity: String,
    pub activity_tipo: String,
    pub stream_link: String,
    pub database: sqlx::SqlitePool
}

impl Handler {
    pub async fn new() -> Self {
        dotenv::dotenv().expect("Não consegui carregar o .env");
        let database = queries::init_db().await;
        
        Self{
            prefixo:env::var("PREFIX").expect("Expected a PREFIX field in the environment"),
            token:env::var("TOKEN").expect("Expected a TOKEN (discord token) field in the environment"),
            //twitter_consumer_key:env::var("TWITTER_CONSUMER_KEY").expect("Expected a TWITTER_CONSUMER_KEY field in the environment"),
            //twitter_consumer_secret:env::var("TWITTER_CONSUMER_SECRET").expect("Expected a TWITTER_CONSUMER_SECRET field in the environment"),
            //twitter_bearer_token:env::var("TWITTER_BEARER_TOKEN").expect("Expected a TWITTER_BEARER_TOKEN field in the environment"),
            //twitter_access_token_key:env::var("TWITTER_ACCESS_TOKEN_KEY").expect("Expected a TWITTER_ACCESS_TOKEN_KEY field in the environment"),
            //twitter_access_token_secret:env::var("TWITTER_ACCESS_TOKEN_SECRET").expect("Expected a TWITTER_ACCESS_TOKEN_SECRET field in the environment"),
            //youtube_api_key:env::var("YOUTUBE_API_KEY").expect("Expected a YOUTUBE_API_KEY field in the environment"),
            //client_idgoogle:env::var("CLIENT_IDGOOGLE").expect("Expected a CLIENT_IDGOOGLE field in the environment"),
            //client_secretgoogle:env::var("CLIENT_SECRETGOOGLE").expect("Expected a CLIENT_SECRETGOOGLE field in the environment"),
            //redirect_urisgoogle:env::var("REDIRECT_URISGOOGLE").expect("Expected a REDIRECT_URISGOOGLE field in the environment"),
            playing_activity:env::var("PLAYING_ACTIVITY").expect("Expected a PLAYING_ACTIVITY field in the environment"),
            activity_tipo:env::var("ACTIVITY_TIPO").expect("Expected a ACTIVITY_TIPO field in the environment"),
            stream_link:env::var("STREAM_LINK").expect("Expected a STREAM_LINK field in the environment"),
            database:database,
        }
    }
    pub fn get_playing_activity_clone(&self) -> String {
        self.playing_activity.clone()
    }
    pub fn get_playing_activity(&self) -> &str {
        self.playing_activity.as_str()
    }
    pub fn get_activity_tipo_clone(&self) -> String {
        self.activity_tipo.clone()
    }
}