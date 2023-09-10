use crate::data::digest::ProductivityData;
use crate::data::website_detection::{build_browser_data_list, BrowserData};
use crate::data::wrappers::WebsiteName;
use crate::data::{Report, Streak};

use super::ProductivityComputation;

#[derive(Default)]
struct CompleteProductivityComputation {
    browsers_data: Vec<BrowserData>,
}

impl CompleteProductivityComputation {
    pub fn new() -> Self {
        Self {
            browsers_data: build_browser_data_list(),
        }
    }
}

impl ProductivityComputation for CompleteProductivityComputation {
    fn compute_productivity(
        &self,
        report: &Report,
        non_productive_apps: &[String],
        non_productive_websites: &[WebsiteName],
    ) -> ProductivityData {
        // TODO : Mutualize this with the one in ProcessNamedProductivityComputation
        struct TmpProductivityData {
            pub total_time: chrono::Duration,
            pub productive_time: chrono::Duration,
        }
        let mut tmp_producivity_data = TmpProductivityData {
            total_time: chrono::Duration::seconds(0),
            productive_time: chrono::Duration::seconds(0),
        };

        let browser_names = self
            .browsers_data
            .iter()
            .map(|b| &b.browser_name)
            .collect::<Vec<_>>();
        let compute_productive_time = |streak: Streak| -> chrono::Duration {
            if self
                .browsers_data
                .iter()
                .map(|d| &d.browser_name)
                .any(|n| streak.process_name.0.contains(n))
            {
                if streak.website_name.as_ref().is_none() {
                    return streak.duration;
                }
            }
            if !non_productive_apps
                .iter()
                .any(|a| streak.process_name.0.contains(a))
            {
                return streak.duration;
            }
            chrono::Duration::seconds(0)
        };

        ProductivityData::default()
    }
}
