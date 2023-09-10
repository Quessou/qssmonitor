mod complete_productivity_computation;
mod process_named_productivity_computation;

use crate::data::{wrappers::WebsiteName, Report};

use super::ProductivityData;

pub trait ProductivityComputation: Default {
    fn compute_productivity(
        &self,
        report: &Report,
        non_productive_apps: &[String],
        non_productive_websites: &[WebsiteName],
    ) -> ProductivityData;
}
