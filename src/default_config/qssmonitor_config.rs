use serde::{Deserialize, Serialize};
use serde_with;

#[serde_with::serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QssMontiorConfig {
    #[serde_as(as = "serde_with::DurationSeconds<i64>")]
    polling_interval: chrono::Duration,
}

impl Default for QssMontiorConfig {
    fn default() -> Self {
        Self {
            polling_interval: chrono::Duration::seconds(5),
        }
    }
}
