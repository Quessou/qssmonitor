use std::str::FromStr;

use super::DetectionDiscriminant;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DetectionCriteria {
    discriminant: DetectionDiscriminant,
    data: String,
}

impl DetectionCriteria {
    pub fn new(discriminant: DetectionDiscriminant, data: &str) -> Self {
        DetectionCriteria {
            discriminant,
            data: data.to_owned(),
        }
    }
    pub fn is_website_detected(&self, window_name: &str) -> bool {
        use DetectionDiscriminant::*;
        match self.discriminant {
            StartsWith => window_name.starts_with(&self.data),
            Contains => window_name.contains(&self.data),
            EndsWith => window_name.ends_with(&self.data),
        }
    }
}

impl From<TomlSerializableDetectionCriteria> for DetectionCriteria {
    fn from(value: TomlSerializableDetectionCriteria) -> Self {
        DetectionCriteria {
            discriminant: DetectionDiscriminant::from_str(&value.discriminant).unwrap(),
            data: value.data,
        }
    }
}

// TODO : Delete me
#[derive(Debug, Serialize, Deserialize)]
pub struct TomlSerializableDetectionCriteria {
    discriminant: String,
    data: String,
}

impl From<DetectionCriteria> for TomlSerializableDetectionCriteria {
    fn from(value: DetectionCriteria) -> Self {
        TomlSerializableDetectionCriteria {
            discriminant: value.discriminant.to_string(),
            data: value.data,
        }
    }
}
