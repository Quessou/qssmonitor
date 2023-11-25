use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct Session {
    pub session_id: i64,
    pub sample_interval: chrono::Duration,
}
