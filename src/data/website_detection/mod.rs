use itertools;
use itertools::Itertools;

use super::wrappers::WebsiteName;
use super::wrappers::WindowName;

pub enum WebsiteNameDetectionCriteria {
    StartsWith(String),
    Contains(String),
    EndsWith(String),
}

impl WebsiteNameDetectionCriteria {
    pub fn is_website_detected(&self, window_name: &str) -> bool {
        use WebsiteNameDetectionCriteria::*;
        match self {
            StartsWith(s) => window_name.starts_with(s),
            Contains(s) => window_name.contains(s),
            EndsWith(s) => window_name.ends_with(s),
        }
    }
}

#[derive(Default)]
pub struct WebsiteNameDetector {
    pub non_productive_websites: Vec<(WebsiteName, Vec<WebsiteNameDetectionCriteria>)>,
}

impl WebsiteNameDetector {
    pub fn get_website_name(&self, window_name: &str) -> Option<WebsiteName> {
        let detected_website_name_data = self
            .non_productive_websites
            .iter()
            .find(|(_, criterias)| criterias.iter().any(|c| c.is_website_detected(window_name)));
        match detected_website_name_data {
            Some(s) => Some(s.0.clone()),
            None => None,
        }
    }
}
