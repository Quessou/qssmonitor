use super::Streak;
use chrono::Duration;

pub struct Report {
    streaks: Vec<Streak>,
    sample_interval: Duration,
    samples_count: u32,
}

impl Report {
    pub fn new(streaks: Vec<Streak>, sample_interval: Duration, samples_count: u32) -> Report {
        Report {
            streaks,
            sample_interval,
            samples_count,
        }
    }
}
