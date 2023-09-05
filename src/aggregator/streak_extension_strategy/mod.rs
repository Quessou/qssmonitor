mod browser_inclusive_streak_extension_strategy;
mod streak_action;
mod website_name_detection_strategy;

pub use streak_action::StreakAction;

use crate::data::{Sample, Streak};

pub trait StreakExtensionStrategy {
    fn get_streak_action(&self, current_streak: &[Sample], next_sample: Sample) -> StreakAction;
}
