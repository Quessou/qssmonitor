use super::StreakRow;
use chrono::Duration;
use sqlx::{sqlite::SqliteRow, FromRow, Row};

pub struct SessionRow {
    id: i64,
    interval: Duration,
    pub(super) streaks: Vec<StreakRow>,
}

impl FromRow<'_, SqliteRow> for SessionRow {
    fn from_row(row: &'_ SqliteRow) -> Result<Self, sqlx::Error> {
        let id: i64 = row.try_get("id").unwrap();
        let interval: chrono::Duration =
            chrono::Duration::seconds(row.try_get::<i64, &str>("sample_interval_s").unwrap());

        Ok(SessionRow {
            id,
            interval,
            streaks: vec![],
        })
    }
}

pub fn insert_streaks(session: &mut SessionRow, streaks: Vec<StreakRow>) {
    session.streaks = streaks;
}
