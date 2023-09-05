use super::StreakAction;
use super::StreakExtensionStrategy;

use crate::data::website_detection::WebsiteNameDetector;
use crate::data::Sample;

pub struct BrowserInclusiveStreakExtensionStrategy {
    pub non_productive_websites: Vec<Vec<WebsiteNameDetector>>,
}
impl BrowserInclusiveStreakExtensionStrategy {}

impl StreakExtensionStrategy for BrowserInclusiveStreakExtensionStrategy {
    fn get_streak_action(
        &self,
        current_streak: &[Sample],
        next_sample: Sample,
    ) -> super::StreakAction {
        // TODO : Test better than just "unwrap"
        let last_sample_website_name = &current_streak.last().unwrap().window_name;
        if *last_sample_website_name == next_sample.window_name {
            return StreakAction::Extend;
        }

        StreakAction::Extend
    }
}
