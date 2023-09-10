use crate::data::digest::ProductivityData;
use crate::data::wrappers::{DurationWrapper, WebsiteName};
use crate::data::Report;

use super::ProductivityComputation;

#[derive(Default)]
pub(crate) struct ProcessNamedProductivityComputation {}

impl ProductivityComputation for ProcessNamedProductivityComputation {
    fn compute_productivity(
        &self,
        report: &Report,
        non_productive_apps: &[String],
        non_productive_websites: &[WebsiteName],
    ) -> ProductivityData {
        //let mut productivity_data = ProductivityData::default();
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
                    if !non_productive_apps
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
                duration: chrono::Duration::seconds(30),
                begin_date: chrono::Local::now(),
            },
            Streak {
                pid: 100,
                process_name: ProcessName("tata".to_owned()),
                window_names: [WindowName("toto".to_owned())].into(),
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
        let computation = ProcessNamedProductivityComputation::default();
        let prod_data = computation.compute_productivity(&report, &vec![] as &Vec<String>);
        assert_eq!(
            prod_data.total_time,
            DurationWrapper {
                duration: chrono::Duration::seconds(50)
            }
        );
        assert_eq!(
            prod_data.productive_time,
            DurationWrapper {
                duration: chrono::Duration::seconds(50)
            }
        );
    }

    #[test]
    fn test_filter_nonproductive_time() {
        let report = build_dummy_report();
        let computation = ProcessNamedProductivityComputation::default();
        let prod_data =
            computation.compute_productivity(&report, &vec!["to".to_owned()] as &Vec<String>);
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
