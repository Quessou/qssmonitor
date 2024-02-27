use chrono::Duration;
use serde::{Deserialize, Serialize};

use super::Streak;
use crate::data::wrappers::DurationWrapper;

use crate::data::wrappers::ProcessName;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
    use std::{collections::HashSet, time::Duration};

    use chrono::DateTime;

    use super::*;
    fn build_simple_streak_vec() -> Vec<Streak> {
        vec![
            Streak {
                pid: 10,
                process_name: "Toto".to_owned().into(),
                window_names: HashSet::default(),
                website_name: None,
                duration: chrono::Duration::seconds(20),
                begin_date: DateTime::default(),
            },
            Streak {
                pid: 10,
                process_name: "Toto".to_owned().into(),
                window_names: HashSet::default(),
                website_name: None,
                duration: chrono::Duration::seconds(30),
                begin_date: DateTime::default(),
            },
            Streak {
                pid: 10,
                process_name: "Toto".to_owned().into(),
                window_names: HashSet::default(),
                website_name: None,
                duration: chrono::Duration::seconds(100),
                begin_date: DateTime::default(),
            },
        ]
    }

    #[test]
    fn test_build_streak_digest() {
        let streaks = build_simple_streak_vec();
        let digest: StreakDigest = streaks.iter().map(|s| s).collect::<Vec<&Streak>>().into();
        assert_eq!(
            digest.average_streak_duration,
            DurationWrapper {
                duration: chrono::Duration::seconds(50)
            }
        );
        assert_eq!(
            digest.longest_streak_duration,
            DurationWrapper {
                duration: chrono::Duration::seconds(100)
            }
        );
    }
}
