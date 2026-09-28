CREATE TABLE tracked_users(
    discord_user_id TEXT PRIMARY KEY,
    username TEXT NOT NULL,
    tracked_since TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);
