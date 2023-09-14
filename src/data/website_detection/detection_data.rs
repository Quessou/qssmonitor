use crate::data::wrappers::WebsiteName;

use super::{DetectionCriteria};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DetectionData {
    pub website_name: WebsiteName,
    pub detection_criterias: Vec<DetectionCriteria>,
}

impl From<(String, Vec<DetectionCriteria>)> for DetectionData {
    fn from(value: (String, Vec<DetectionCriteria>)) -> Self {
        DetectionData {
            website_name: value.0.into(),
            detection_criterias: value.1,
        }
    }
}
impl From<(WebsiteName, Vec<DetectionCriteria>)> for DetectionData {
    fn from(value: (WebsiteName, Vec<DetectionCriteria>)) -> Self {
        DetectionData {
            website_name: value.0,
            detection_criterias: value.1,
        }
    }
}
/*
#[derive(Debug, Serialize, Deserialize)]
pub struct TomlSerializableDetectionData {
    pub website_name: String,
    pub detection_criterias: Option<Vec<TomlSerializableDetectionCriteria>>,
}

impl From<DetectionData> for TomlSerializableDetectionData {
    fn from(value: DetectionData) -> Self {
        Self {
            website_name: value.website_name.data,
            detection_criterias: Some(
                value
                    .detection_criterias
                    .into_iter()
                    .map(|c| c.into())
                    .collect::<Vec<_>>(),
            ),
        }
    }
}

impl From<TomlSerializableDetectionData> for DetectionData {
    fn from(value: TomlSerializableDetectionData) -> Self {
        Self {
            website_name: WebsiteName::from(value.website_name),
            detection_criterias: value
                .detection_criterias
                .unwrap()
                .into_iter()
                .map(|c| c.into())
                .collect::<Vec<_>>(),
        }
    }
}
*/
