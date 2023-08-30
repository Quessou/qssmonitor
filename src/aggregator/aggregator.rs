use chrono::Duration;

use crate::data::sample::Sample;
use crate::data::Report;
use crate::data::Streak;

pub struct Aggregator {
    sample_interval: Duration,
    streaks: Vec<Streak>,
    current_streak: Vec<Sample>,
    stored_samples_count: u32,
}

impl Aggregator {
    pub fn new(sample_interval: Duration) -> Self {
        Aggregator {
            sample_interval,
            streaks: vec![],
            current_streak: vec![],
            stored_samples_count: 0,
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
        if self.current_streak.is_empty() || self.current_streak[0].pid == sample.pid {
            self.extend_streak(sample)
        } else if !self.current_streak.is_empty() {
            self.register_streak()
        } else {
            self.extend_streak(sample)
        }
    }

    pub fn register_sample(&mut self, sample: Sample) {
        self.update_streaks(&sample).unwrap();
        self.stored_samples_count += 1;
    }

    pub fn get_report(&mut self) -> Report {
        let mut streaks = self.streaks.clone();
        let last_streak = (self.current_streak.clone(), self.sample_interval).into();
        streaks.push(last_streak);
        Report::new(streaks, self.sample_interval, self.stored_samples_count)
    }
}
