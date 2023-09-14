use serde::{Deserialize, Serialize};

use crate::data::website_detection::DetectionData;

use super::NonProductiveWebsitesConfiguration;

#[serde_with::serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QssMonitorConfig {
    #[serde_as(as = "serde_with::DurationSeconds<i64>")]
    polling_interval: chrono::Duration,
    non_productive_apps: Vec<String>,
    pub non_productive_website: Vec<DetectionData>, //NonProductiveWebsitesConfiguration,
}

impl Default for QssMonitorConfig {
    fn default() -> Self {
        Self {
            polling_interval: chrono::Duration::seconds(5),
            non_productive_apps: vec![],
            non_productive_website: NonProductiveWebsitesConfiguration::default()
                .non_productive_websites,
        }
    }
}
