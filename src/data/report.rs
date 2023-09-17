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
}

impl std::string::ToString for Report {
    fn to_string(&self) -> String {
        format!("{}", self.samples_count)
    }
}

/*
impl Into<Response> for Report {
    fn into(self) -> Response {
        self.to_string().into()
    }
}
*/
