mod browser_data;
mod detection_criteria;
mod detection_data;
mod detection_discriminant;

use super::wrappers::WebsiteName;

pub(crate) use browser_data::BrowserData;
pub(crate) use detection_criteria::{DetectionCriteria, TomlSerializableDetectionCriteria};
pub(crate) use detection_data::DetectionData;
pub(crate) use detection_discriminant::DetectionDiscriminant;

#[derive(Default)]
pub struct WebsiteNameDetector {
    pub non_productive_websites: Vec<DetectionData>,
    pub browser_data: Vec<BrowserData>,
}

impl WebsiteNameDetector {
    pub fn new(non_productive_websites: Vec<DetectionData>) -> Self {
        Self {
            non_productive_websites,
            browser_data: build_browser_data_list(),
        }
    }
    pub fn get_website_name(&self, process_name: &str, window_name: &str) -> Option<WebsiteName> {
        let browser_data: Option<&BrowserData> = self
            .browser_data
            .iter()
            .find(|d| process_name.ends_with(&d.browser_name));
        let browser_data = match browser_data {
            None => return None,
            Some(b) => b,
        };
        let _binding = window_name.replace(&browser_data.window_name_suffix, "");
        let cleared_window_name = _binding.trim();
        let detected_website_name_data = self.non_productive_websites.iter().find(|data| {
            data.detection_criterias
                .iter()
                .any(|c| c.is_website_detected(cleared_window_name))
        });
        match detected_website_name_data {
            Some(s) => Some(s.website_name.clone()),
            None => None,
        }
    }
}

pub fn build_browser_data_list() -> Vec<BrowserData> {
    return vec![
        BrowserData::new("firefox".to_owned(), "— Mozilla Firefox".to_owned()),
        BrowserData::new("chrome".to_owned(), "TODO".to_owned()),
        BrowserData::new("chromium".to_owned(), "TODO".to_owned()),
    ];
}
