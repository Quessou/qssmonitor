use chrono::TimeZone;
use sqlx::{error::Error, sqlite::SqliteRow, FromRow, Row};

use crate::data::wrappers::{ProcessName, WebsiteName};

/// Streak data that is read from a database
/// Less detailed than a Streak, but theorically contain all the data we need
pub struct StreakRow {
    id: i64,
    process_name: ProcessName,
    website_name: Option<WebsiteName>,
    begin_date: chrono::DateTime<chrono::offset::Local>,
    duration: chrono::Duration,
}

impl FromRow<'_, SqliteRow> for StreakRow {
    fn from_row(row: &SqliteRow) -> Result<Self, Error> {
        let id: i64 = row.try_get("id").unwrap();
        let process_name: ProcessName = row.try_get::<String, &str>("process_name").unwrap().into();
        let website_name = match row.try_get::<String, &str>("website_name") {
            Ok(r) => Some(r.into()),
            Err(_) => None,
        };
        let begin_date = chrono::offset::Local
            .timestamp_opt(row.try_get::<i64, &str>("begin_date").unwrap().into(), 0)
            .unwrap();
        let duration: chrono::Duration =
            chrono::Duration::seconds(row.try_get::<i64, &str>("duration_s").unwrap());

        Ok(StreakRow {
            id,
            process_name,
            website_name,
            begin_date,
            duration,
        })
    }
}
