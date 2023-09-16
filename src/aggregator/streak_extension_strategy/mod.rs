mod browser_inclusive_streak_extension_strategy;
mod streak_action;
mod website_name_detection_strategy;

pub(crate) use browser_inclusive_streak_extension_strategy::BrowserInclusiveStreakExtensionStrategy;
pub use streak_action::StreakAction;

use crate::data::Sample;

pub trait StreakExtensionStrategy: Send + std::fmt::Debug {
    fn get_streak_action(&self, current_streak: &[Sample], next_sample: &Sample) -> StreakAction;
}
/*
impl std::fmt::Debug for dyn StreakExtensionStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
*/
