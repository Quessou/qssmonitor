use super::StreakRow;
use chrono::Duration;
use sqlx::{sqlite::SqliteRow, FromRow, Row};

pub struct SessionRow {
    id: i64,
    interval: Duration,
    streaks: Vec<StreakRow>,
}

impl FromRow<'_, SqliteRow> for SessionRow {
    fn from_row(row: &'_ SqliteRow) -> Result<Self, sqlx::Error> {
        let id: i64 = row.try_get("id").unwrap();
        let interval: chrono::Duration =
            chrono::Duration::seconds(row.try_get::<i64, &str>("sample_interval_s").unwrap());
        /*
        let streaks_ids = match sqlx::query_as::<Sqlite, i64>(
            "SELECT id_streak FROM session_streak WHERE id_session = ?",
        )
        .bind(id)
        .fetch_all(&mut *conn)
        .await
        {
            Ok(r) => {}
            Err(_) => {}
        };
        */

        Ok(SessionRow {
            id,
            interval,
            streaks: vec![],
        })
    }
}
