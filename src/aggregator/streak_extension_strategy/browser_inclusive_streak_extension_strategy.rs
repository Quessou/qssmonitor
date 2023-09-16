use super::StreakAction;
use super::StreakExtensionStrategy;

use crate::data::website_detection::{build_browser_data_list, BrowserData};
use crate::data::Sample;

#[derive(Default, Debug)]
pub struct BrowserInclusiveStreakExtensionStrategy {
    browser_data_list: Vec<BrowserData>,
}
impl BrowserInclusiveStreakExtensionStrategy {
    pub fn new() -> Self {
        Self {
            browser_data_list: build_browser_data_list(),
        }
    }
}

impl StreakExtensionStrategy for BrowserInclusiveStreakExtensionStrategy {
    fn get_streak_action(
        &self,
        current_streak: &[Sample],
        next_sample: &Sample,
    ) -> super::StreakAction {
        if current_streak.is_empty() {
            return StreakAction::Extend;
        }
        if current_streak.last().unwrap().process_name != next_sample.process_name {
            return StreakAction::RegisterAndExtend;
        }
        if self
            .browser_data_list
            .iter()
            .map(|d| &d.browser_name)
            .any(|name| next_sample.process_name.0.ends_with(name))
        {
            let last_sample_website_name = &current_streak.last().unwrap().website_name;
            if *last_sample_website_name == next_sample.website_name {
                return StreakAction::Extend;
            }

            StreakAction::RegisterAndExtend
        } else {
            StreakAction::Extend
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::data::wrappers::{ProcessName, WebsiteName, WindowName};
    use test_case::test_case;

    use super::*;

    fn build_sample(
        process_name: ProcessName,
        window_name: WindowName,
        website_name: Option<WebsiteName>,
        pid: i32,
    ) -> Sample {
        Sample::new(process_name, window_name, website_name, pid)
    }

    #[test_case(build_sample(ProcessName { 0 : "toto".to_owned()}, WindowName { 0: "toto".to_owned()}, None, 10),
                           build_sample(ProcessName { 0 : "tata".to_owned()}, WindowName { 0: "toto".to_owned()}, None, 10) 
                           => StreakAction::RegisterAndExtend;
                           "Two samples who do not have the same process name")]
    #[test_case(build_sample(ProcessName { 0 : "firefox".to_owned()}, WindowName { 0: "toto".to_owned()}, Some(WebsiteName{ 0 : "website".to_owned()}), 10),
                           build_sample(ProcessName { 0 : "firefox".to_owned()}, WindowName { 0: "toto".to_owned()}, Some(WebsiteName{ 0 : "website".to_owned()}), 10) 
                           => StreakAction::Extend;
                           "Two browser processes and the same website name")]
    #[test_case(build_sample(ProcessName { 0 : "firefox".to_owned()}, WindowName { 0: "toto".to_owned()}, Some(WebsiteName{ 0 : "website_1".to_owned()}), 10),
                           build_sample(ProcessName { 0 : "firefox".to_owned()}, WindowName { 0: "toto".to_owned()}, Some(WebsiteName{ 0 : "website_2".to_owned()}), 10) 
                           => StreakAction::RegisterAndExtend;
                           "Switching between tabs should break the streak")]
    #[test_case(build_sample(ProcessName { 0 : "firefox".to_owned()}, WindowName { 0: "toto".to_owned()}, Some(WebsiteName{ 0 : "website_1".to_owned()}), 10),
                           build_sample(ProcessName { 0 : "toto".to_owned()}, WindowName { 0: "toto".to_owned()}, None, 10) 
                           => StreakAction::RegisterAndExtend;
                           "Switching back to a nonbrowser app should break the streak")]
    fn test_get_streak_action(previous_sample: Sample, next_sample: Sample) -> StreakAction {
        let strategy = BrowserInclusiveStreakExtensionStrategy::new();
        strategy.get_streak_action(&vec![previous_sample], &next_sample)
    }
}
