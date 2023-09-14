use crate::data::digest::ProductivityData;
use crate::data::website_detection::{BrowserData};
use crate::data::wrappers::{DurationWrapper};
use crate::data::{Report, Streak};

use super::ProductivityComputation;

#[derive(Default)]
struct CompleteProductivityComputation {
    browsers_data: Vec<BrowserData>,
}

impl CompleteProductivityComputation {
    pub fn new(browsers_data: Vec<BrowserData>) -> Self {
        Self { browsers_data }
    }
}

impl ProductivityComputation for CompleteProductivityComputation {
    fn compute_productivity(
        &self,
        report: &Report,
        non_productive_apps: &[String],
    ) -> ProductivityData {
        let browsers_names = self
            .browsers_data
            .iter()
            .map(|b| &b.browser_name)
            .collect::<Vec<_>>();
        let compute_productive_time = |streak: &Streak| -> chrono::Duration {
            if browsers_names
                .iter()
                .any(|&n| streak.process_name.0.contains(n))
            {
                if streak.website_name.as_ref().is_none() {
                    return streak.duration;
                }
            } else if !non_productive_apps
                .iter()
                .any(|a| streak.process_name.0.contains(a))
            {
                return streak.duration;
            }
            chrono::Duration::seconds(0)
        };

        let productive_time = report
            .streaks
            .iter()
            .map(compute_productive_time)
            .fold(chrono::Duration::seconds(0), |acc, d| acc + d);
        let total_time = report
            .streaks
            .iter()
            .fold(chrono::Duration::seconds(0), |acc, s| acc + s.duration);
        ProductivityData {
            total_time: DurationWrapper {
                duration: total_time,
            },
            productive_time: DurationWrapper {
                duration: productive_time,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::{
        wrappers::{ProcessName, WindowName},
        Sample,
    };
    use test_case::test_case;

    fn build_sample(process_name: &str, window_name: &str, website_name: &str, pid: i32) -> Sample {
        Sample::new(
            ProcessName {
                0: process_name.to_owned(),
            },
            WindowName {
                0: window_name.to_owned(),
            },
            Some(WebsiteName {
                0: website_name.to_owned(),
            }),
            pid,
        )
    }

    #[test_case(Report::new(
            vec![
            (vec![build_sample("toto", "toto", "mdr", 10), build_sample("toto", "toto", "mdr", 10)], 
             chrono::Duration::seconds(5)).into()], chrono::Duration::seconds(5), 2 as u32) => chrono::Duration::seconds(0) )]
    #[test_case(Report::new(
            vec![
            (vec![build_sample("toto", "toto", "mdr", 10), build_sample("toto", "toto", "mdr", 10)], 
             chrono::Duration::seconds(5)).into(),(vec![build_sample("tutu", "toto", "", 10), build_sample("tutu", "toto", "mdr", 10)], 
             chrono::Duration::seconds(5)).into()], chrono::Duration::seconds(5), 2 as u32) => chrono::Duration::seconds(10) )]
    fn test_compute_productivity(report: Report) -> chrono::Duration {
        let computation = CompleteProductivityComputation::new(vec![BrowserData {
            browser_name: "toto".to_owned(),
            window_name_suffix: "".to_owned(),
        }]);
        computation
            .compute_productivity(&report, &vec![])
            .productive_time
            .duration
    }
}
