use std::ops::Add;

use serde::{Deserialize, Serialize};
use serde_with;

#[serde_with::serde_as]
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct DurationWrapper {
    #[serde_as(as = "serde_with::DurationSeconds<i64>")]
    pub duration: chrono::Duration,
}

impl Add for DurationWrapper {
    type Output = DurationWrapper;

    fn add(self, rhs: Self) -> Self::Output {
        DurationWrapper {
            duration: self.duration + rhs.duration,
        }
    }
}

impl Default for DurationWrapper {
    fn default() -> Self {
        Self {
            duration: chrono::Duration::seconds(0),
        }
    }
}
