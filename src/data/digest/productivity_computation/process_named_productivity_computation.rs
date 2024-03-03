use crate::data::digest::ProductivityData;
use crate::data::wrappers::DurationWrapper;
use crate::data::Report;

use super::ProductivityComputation;

/// Way to compute productivity that only relies on the process name (the stuff that is returned by
/// ps, basically). So it's kinda inaccurate since it does not take into consideration what we do
/// in our browser, for instance.
#[derive(Debug, Default, Clone)]
pub(crate) struct ProcessNamedProductivityComputation {
    non_productive_apps: Vec<String>,
}

unsafe impl std::marker::Send for ProcessNamedProductivityComputation {}
unsafe impl std::marker::Sync for ProcessNamedProductivityComputation {}

impl ProcessNamedProductivityComputation {
    #[allow(dead_code)]
    pub fn new(non_productive_apps: Vec<String>) -> Self {
        Self {
            non_productive_apps,
        }
    }
}

impl ProductivityComputation for ProcessNamedProductivityComputation {
    fn compute_productivity(&self, report: &Report) -> ProductivityData {
        struct TmpProductivityData {
            pub total_time: chrono::Duration,
            pub productive_time: chrono::Duration,
        }
        let mut tmp_producivity_data = TmpProductivityData {
            total_time: chrono::Duration::seconds(0),
            productive_time: chrono::Duration::seconds(0),
        };
        let tmp_productivity_data =
            report
                .streaks
                .iter()
                .fold(&mut tmp_producivity_data, |acc, s| {
                    acc.total_time = acc.total_time + s.duration;
                    if !self
                        .non_productive_apps
                        .iter()
                        .any(|a| s.process_name.0.contains(a))
                    {
                        acc.productive_time = acc.productive_time + s.duration;
                    }
                    acc
                });
        ProductivityData {
            total_time: DurationWrapper {
                duration: tmp_productivity_data.total_time,
            },
            productive_time: DurationWrapper {
                duration: tmp_productivity_data.productive_time,
            },
            pause_time: 0.into(), // Who cares, this class is actually deprecated
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::data::{wrappers::ProcessName, wrappers::WindowName, Streak};

    fn build_dummy_report() -> Report {
        let streaks: Vec<Streak> = vec![
            Streak {
                pid: 10,
                process_name: ProcessName("toto".to_owned()),
                window_names: [WindowName("toto".to_owned())].into(),
                website_name: None,
                duration: chrono::Duration::seconds(30),
                begin_date: chrono::Local::now(),
            },
            Streak {
                pid: 100,
                process_name: ProcessName("tata".to_owned()),
                window_names: [WindowName("toto".to_owned())].into(),
                website_name: None,
                duration: chrono::Duration::seconds(20),
                begin_date: chrono::Local::now() + chrono::Duration::seconds(30),
            },
        ];
        let sample_interval = chrono::Duration::seconds(5);
        let samples_count: u32 = 10;
        Report::new(streaks, sample_interval, samples_count)
    }

    #[test]
    fn test_compute_productivity() {
        let report = build_dummy_report();
        let computation = ProcessNamedProductivityComputation::new(vec!["to".to_owned()]);
        let prod_data = computation.compute_productivity(&report);
        assert_eq!(
            prod_data.total_time,
            DurationWrapper {
                duration: chrono::Duration::seconds(50)
            }
        );
        assert_eq!(
            prod_data.productive_time,
            DurationWrapper {
                duration: chrono::Duration::seconds(20)
            }
        );
    }

    #[test]
    fn test_filter_nonproductive_time() {
        let report = build_dummy_report();
        let computation = ProcessNamedProductivityComputation {
            non_productive_apps: vec!["to".to_owned()],
        };
        let prod_data = computation.compute_productivity(&report);
        assert_eq!(
            prod_data.total_time,
            DurationWrapper {
                duration: chrono::Duration::seconds(50)
            }
        );
        assert_eq!(
            prod_data.productive_time,
            DurationWrapper {
                duration: chrono::Duration::seconds(20)
            }
        );
    }
}
