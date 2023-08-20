use serde::{Deserialize, Serialize};


#[serde_with::serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QssMonitorConfig {
    #[serde_as(as = "serde_with::DurationSeconds<i64>")]
    polling_interval: chrono::Duration,
}

impl Default for QssMonitorConfig {
    fn default() -> Self {
        Self {
            polling_interval: chrono::Duration::seconds(5),
        }
    }
}
