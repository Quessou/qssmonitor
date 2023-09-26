extern crate alloc;
use alloc::borrow::Cow;
use sqlx::migrate::{Migration, MigrationType};

pub fn get_migrations() -> Vec<Migration> {
    vec![Migration::new(
        0,
        "First migration with Streaks and reports".into(),
        MigrationType::Simple,
        r#"
CREATE TABLE table_report(
    id INTEGER NOT NULL,
    PRIMARY KEY (id)
);
CREATE TABLE table_streak(
    id INTEGER NOT NULL,
    PRIMARY KEY (id),
    process_name TEXT NOT NULL,
    website_name TEXT,
    begin_date INTEGER NOT NULL,
    duration_s INTEGER NOT NULL
);
CREATE TABLE report_streak(
    report_id INTEGER NOT NULL,
    streak_id INTEGER NOT NULL
    PRIMARY_KEY (report_id, streak_id)
);
"#
        .into(),
    )]
}
