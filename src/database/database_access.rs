use async_trait::async_trait;

use crate::data::Streak;

#[async_trait]
pub trait DatabaseAccess: Send + Clone {
    async fn create_session(&self, duration_interval: chrono::Duration) -> Result<i64, ()>;
    async fn save_streak(&self, streak: &Streak, session_id: i64) -> Result<i64, ()>;
}
