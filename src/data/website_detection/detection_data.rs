use crate::data::wrappers::WebsiteName;

use super::DetectionCriteria;
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
