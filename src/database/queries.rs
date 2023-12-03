use serenity::model::prelude::Message;
use crate::handler::handler::Handler;
use crate::database::mensagens::Mensagem;
//use serenity::model::application::interaction::Interaction;

pub async fn log_message(message: &Message, handler: &Handler) -> sqlx::Result<sqlx::sqlite::SqliteQueryResult> {
    let msg = Mensagem::new_from_message(message);
    sqlx::query!(
        "INSERT into mensagens(mensagem, idutilizador, nomeutilizador, datacriacao, attachments_json, tipo_mensagem, channel_id) values (?,?,?,?,?,?,?);",
        msg.mensagem, msg.idutilizador, msg.nomeutilizador,msg.datacriacao, msg.attachments_json, msg.tipo_mensagem, msg.channel_id
    ).execute(&handler.database).await
}
//todo
/*pub async fn insert_interaction(interaction: &Interaction, handler: &Handler) -> sqlx::Result<sqlx::sqlite::SqliteQueryResult> {
    match Mensagem::new_from_interaction(interaction){
        Some(msg) => 
    };
}*/

pub async fn init_db() -> sqlx::sqlite::SqlitePool {
    let fname = match std::env::var("DATABASE_URL") {
        Ok(n) => n,
        Err(e) => panic!("Couldnt read env var DATABASE_URL, {}", e)
    };
    let database = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(
            sqlx::sqlite::SqliteConnectOptions::new()
                .filename(fname)
                .create_if_missing(true),
        )
        .await
        .expect("Couldn't connect to database");
        
        //obter query do ficheiro e executar
        /*let query_str = match std::fs::read_to_string("schema.sql") {
            Ok(string) => string,
            Err(e) => panic!("Could not read schema.sql: {:?}", e)
        };

        match sqlx::query!("SELECT 1 FROM schema; ?",query_str).execute(&database).await {
            Ok(_) => {},
            Err(e) => panic!("Could not execute schema.sql: {:?}", e)
        }*/
        sqlx::migrate!("./migrations").run(&database).await.expect("Couldn't run database migrations");
        database
}