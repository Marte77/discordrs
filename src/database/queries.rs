use crate::database::mensagens::Mensagem;
use crate::handler::handler::Handler;
use serenity::model::prelude::Message;
use sqlx::Row;
//use serenity::model::application::interaction::Interaction;

pub async fn log_message(
    message: &Message,
    handler: &Handler,
) -> sqlx::Result<sqlx::sqlite::SqliteQueryResult> {
    let msg = Mensagem::new_from_message(message);
    sqlx::query!(
        "INSERT into mensagens(mensagem, idutilizador, nomeutilizador, datacriacao, attachments_json, tipo_mensagem, channel_id) values (?,?,?,?,?,?,?);",
        msg.mensagem, msg.idutilizador, msg.nomeutilizador,msg.datacriacao, msg.attachments_json, msg.tipo_mensagem, msg.channel_id
    ).execute(&handler.database).await
}

pub async fn insert_tracked_pfp(
    discord_user_id: &str,
    username: &str,
    avatar_url: &str,
    avatar_base64: &str,
    handler: &Handler,
) -> sqlx::Result<sqlx::sqlite::SqliteQueryResult> {
    sqlx::query(
        "INSERT INTO tracked_profile_pictures(discord_user_id, username, avatar_url, avatar_base64) VALUES (?,?,?,?);",
    )
    .bind(discord_user_id)
    .bind(username)
    .bind(avatar_url)
    .bind(avatar_base64)
    .execute(&handler.database)
    .await
}

pub async fn upsert_tracked_user(
    discord_user_id: &str,
    username: &str,
    handler: &Handler,
) -> sqlx::Result<sqlx::sqlite::SqliteQueryResult> {
    sqlx::query(
        "INSERT INTO tracked_users(discord_user_id, username) VALUES (?,?)
         ON CONFLICT(discord_user_id) DO UPDATE SET username=excluded.username;",
    )
    .bind(discord_user_id)
    .bind(username)
    .execute(&handler.database)
    .await
}

pub async fn is_user_tracked(discord_user_id: &str, handler: &Handler) -> sqlx::Result<bool> {
    let row = sqlx::query("SELECT 1 FROM tracked_users WHERE discord_user_id = ? LIMIT 1;")
        .bind(discord_user_id)
        .fetch_optional(&handler.database)
        .await?;
    Ok(row.is_some())
}

pub async fn latest_tracked_avatar_url(
    discord_user_id: &str,
    handler: &Handler,
) -> sqlx::Result<Option<String>> {
    let row = sqlx::query(
        "SELECT avatar_url FROM tracked_profile_pictures
         WHERE discord_user_id = ?
         ORDER BY id DESC LIMIT 1;",
    )
    .bind(discord_user_id)
    .fetch_optional(&handler.database)
    .await?;

    match row {
        Some(row) => row.try_get::<String, _>("avatar_url").map(Some),
        None => Ok(None),
    }
}

pub async fn get_tracked_user_ids(handler: &Handler) -> sqlx::Result<Vec<String>> {
    let rows = sqlx::query("SELECT discord_user_id FROM tracked_users;")
        .fetch_all(&handler.database)
        .await?;

    let mut ids = Vec::with_capacity(rows.len());
    for row in rows {
        ids.push(row.try_get::<String, _>("discord_user_id")?);
    }
    Ok(ids)
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
        Err(e) => panic!("Couldnt read env var DATABASE_URL, {}", e),
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
    sqlx::migrate!("./migrations")
        .run(&database)
        .await
        .expect("Couldn't run database migrations");
    database
}
