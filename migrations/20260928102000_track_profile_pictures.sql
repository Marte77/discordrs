CREATE TABLE tracked_profile_pictures(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    discord_user_id TEXT NOT NULL,
    username TEXT NOT NULL,
    avatar_url TEXT NOT NULL,
    tracked_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);
