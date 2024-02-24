// TODO : Remove me

use super::Streak;
use chrono::Duration;

use serde::{Deserialize, Serialize};
use serde_with::serde_as;

/// Compilation of a set of `Streak`
#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct Report {
    pub streaks: Vec<Streak>,
    #[serde_as(as = "serde_with::DurationSeconds<i64>")]
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

    /// Returns the longest streaks of the report
    pub fn get_longest_streaks(&self) -> Option<Vec<Streak>> {
        if self.streaks.is_empty() {
            return None;
        }
        let max_duration = self.streaks.iter().map(|s| s.duration).max().unwrap();
        let longest_streaks = self
            .streaks
            .iter()
            .filter(|s| s.duration == max_duration)
            .cloned()
            .collect::<Vec<Streak>>();
        assert!(!longest_streaks.is_empty());
        Some(longest_streaks)
    }
}

impl std::string::ToString for Report {
    fn to_string(&self) -> String {
        format!("{}", self.samples_count)
    }
}
