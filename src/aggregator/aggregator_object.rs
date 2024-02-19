use chrono::Duration;

use crate::data::sample::Sample;
use crate::data::Report;
use crate::data::Streak;
use crate::database::DatabaseAccess;

use super::session::Session;
use super::streak_extension_strategy;
use super::streak_extension_strategy::StreakExtensionStrategy;

/// Handles aggregation of samples in streaks
/// When a streak is finished, stores it in DB, attaching it to current session
#[derive(Debug)]
pub struct Aggregator<T: DatabaseAccess> {
    sample_interval: Duration,
    streaks: Vec<Streak>,
    current_streak: Vec<Sample>,
    stored_samples_count: u32,
    streak_extension_strategy: Box<dyn StreakExtensionStrategy + Send>,
    db_access: T,
    current_session: Option<Session>,
}

impl<DB: DatabaseAccess> Aggregator<DB> {
    pub fn new(
        sample_interval: Duration,
        streak_extension_strategy: Box<dyn StreakExtensionStrategy + Send>,
        db_access: DB,
    ) -> Self {
        Aggregator {
            sample_interval,
            streaks: vec![],
            current_streak: vec![],
            stored_samples_count: 0,
            streak_extension_strategy,
            db_access,
            current_session: None,
        }
    }

    fn extend_streak(&mut self, sample: &Sample) -> Result<(), ()> {
        self.current_streak.push(sample.clone());
        Ok(())
    }

    async fn register_streak(&mut self) -> Result<(), ()> {
        if self.current_streak.is_empty() {
            tracing::warn!("Trying to register an empty sample streak");
            return Err(());
        }

        let mut current_streak = vec![];
        std::mem::swap(&mut current_streak, &mut self.current_streak);
        let streak = (current_streak, self.sample_interval).into();
        self.db_access
            .save_streak(&streak, self.current_session.as_ref().unwrap().session_id)
            .await
            .expect("Streak saving in DB failed");
        self.streaks.push(streak);
        Ok(())
    }

    /// Function to call when a new sample is received, to either append it to the current streak
    /// Or save the current streak in database and start a new one.
    async fn update_streaks(&mut self, sample: &Sample) -> Result<(), ()> {
        let streak_action = self
            .streak_extension_strategy
            .get_streak_action(&self.current_streak, sample);
        let r = if let streak_extension_strategy::StreakAction::RegisterAndExtend = streak_action {
            self.register_streak().await
        } else {
            Ok(())
        };
        r?;
        self.extend_streak(sample)
    }

    pub async fn register_sample(&mut self, sample: Sample) {
        tracing::info!("{:?}", sample);
        self.update_streaks(&sample).await.unwrap();
        self.stored_samples_count += 1;
    }

    pub fn get_current_report(&mut self) -> Report {
        let mut streaks = self.streaks.clone();
        let last_streak = (self.current_streak.clone(), self.sample_interval).into();
        streaks.push(last_streak);
        Report::new(streaks, self.sample_interval, self.stored_samples_count)
    }

    pub async fn start_session(&mut self) -> Result<(), ()> {
        let db_access = &self.db_access;
        match db_access.create_session(self.sample_interval).await {
            Ok(id) => {
                self.current_session = Some(Session {
                    session_id: id,
                    sample_interval: self.sample_interval,
                });
                Ok(())
            }
            Err(e) => {
                tracing::error!("Could not create session : {:?}", e);
                Err(())
            }
        }
    }
}
