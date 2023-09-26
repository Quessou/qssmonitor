use chrono::Duration;

use crate::data::sample::Sample;
use crate::data::Report;
use crate::data::Streak;
use crate::database::DatabaseAccess;

use super::session::Session;
use super::streak_extension_strategy;
use super::streak_extension_strategy::StreakExtensionStrategy;

#[derive(Debug)]
pub struct Aggregator<T: DatabaseAccess> {
    sample_interval: Duration,
    streaks: Vec<Streak>,
    current_streak: Vec<Sample>,
    stored_samples_count: u32,
    streak_extension_strategy: Box<dyn StreakExtensionStrategy>,
    db_access: T,
    current_session: Option<Session>,
}

impl<DB: DatabaseAccess> Aggregator<DB> {
    pub fn new(
        sample_interval: Duration,
        streak_extension_strategy: Box<dyn StreakExtensionStrategy>,
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

    fn register_streak(&mut self) -> Result<(), ()> {
        if self.current_streak.is_empty() {
            tracing::warn!("Trying to register an empty sample streak");
            return Err(());
        }

        let mut current_streak = vec![];
        std::mem::swap(&mut current_streak, &mut self.current_streak);
        let streak = (current_streak, self.sample_interval).into();
        self.streaks.push(streak);
        Ok(())
    }

    fn update_streaks(&mut self, sample: &Sample) -> Result<(), ()> {
        let r = if let streak_extension_strategy::StreakAction::RegisterAndExtend = self
            .streak_extension_strategy
            .get_streak_action(&self.current_streak, sample)
        {
            self.register_streak()
        } else {
            Ok(())
        };
        r?;
        self.extend_streak(sample)
    }

    pub fn register_sample(&mut self, sample: Sample) {
        tracing::info!("{:?}", sample);
        self.update_streaks(&sample).unwrap();
        self.stored_samples_count += 1;
    }

    pub fn get_report(&mut self) -> Report {
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
