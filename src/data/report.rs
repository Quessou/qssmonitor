use super::Streak;
use chrono::Duration;

/// Compilation of a set of `Streak`
pub struct Report {
    pub streaks: Vec<Streak>,
    pub sample_interval: Duration,
    pub samples_count: u32,
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
