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
