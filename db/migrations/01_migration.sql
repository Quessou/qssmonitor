CREATE TABLE IF NOT EXISTS table_session(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    sample_interval_s INTEGER NOT NULL
);
CREATE TABLE IF NOT EXISTS table_streak(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    process_name TEXT NOT NULL,
    website_name TEXT,
    begin_date INTEGER NOT NULL,
    duration_s INTEGER NOT NULL
);
CREATE TABLE IF NOT EXISTS session_streak(
    session_id INTEGER NOT NULL,
    streak_id INTEGER NOT NULL,
    PRIMARY KEY (session_id, streak_id)
);
