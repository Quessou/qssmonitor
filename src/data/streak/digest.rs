use chrono::Duration;
use serde::{Deserialize, Serialize};

use super::Streak;
use crate::data::wrappers::DurationWrapper;

use crate::process::ProcessName;

#[derive(Serialize, Deserialize)]
pub struct StreakDigest {
    pub process_name: ProcessName,
    pub longest_streak_duration: DurationWrapper,
    pub average_streak_duration: DurationWrapper,
}

impl From<Vec<&Streak>> for StreakDigest {
    fn from(streaks: Vec<&Streak>) -> Self {
        let mut longest_duration: Duration = Duration::seconds(0);
        let total_duration =
            streaks
                .iter()
                .map(|s| s.duration)
                .fold(Duration::seconds(0), |acc, d| {
                    if d > longest_duration {
                        longest_duration = d;
                    }
                    acc + d
                });
        let average_duration = total_duration / (streaks.len() as i32);
        StreakDigest {
            process_name: streaks[0].process_name.clone(),
            longest_streak_duration: DurationWrapper {
                duration: longest_duration,
            },
            average_streak_duration: DurationWrapper {
                duration: average_duration,
            },
        }
    }
}
