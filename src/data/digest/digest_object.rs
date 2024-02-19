use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::data::streak::StreakDigest;
use crate::data::wrappers::DurationWrapper;
use crate::data::wrappers::ProcessName;
use crate::data::Streak;

use crate::data::report::Report;

use super::productivity_computation::ProductivityComputation;
use super::productivity_data;
use super::ProductivityData;

/// Compilation of data (*with processings, so there is some data loss*) that can be requested from
/// an outside client
#[serde_with::serde_as]
#[derive(Serialize, Deserialize)]
pub struct Digest {
    pub begin_date: chrono::DateTime<chrono::Local>,
    pub end_date: chrono::DateTime<chrono::Local>,
    pub time_by_process: Vec<(ProcessName, DurationWrapper)>,
    pub streak_data: Vec<StreakDigest>,
    pub productivity_data: Option<ProductivityData>,
}

fn group_streaks_by_process_name(streaks: &[Streak]) -> Vec<Vec<&Streak>> {
    let mut grouped_streaks: Vec<Vec<&Streak>> = vec![];
    for (_, group) in &streaks.iter().group_by(|s| &s.process_name) {
        grouped_streaks.push(group.collect());
    }
    grouped_streaks
}

fn get_begin_and_end_dates(
    report: &Report,
) -> (
    chrono::DateTime<chrono::Local>,
    chrono::DateTime<chrono::Local>,
) {
    let begin_date = report.streaks[0].begin_date;
    let duration: chrono::Duration = report
        .streaks
        .iter()
        .fold(chrono::Duration::seconds(0), |i, s| i + s.duration);
    let end_date = begin_date + duration;
    (begin_date, end_date)
}

fn aggregate_durations(grouped_streaks: &[Vec<&Streak>]) -> Vec<(ProcessName, DurationWrapper)> {
    grouped_streaks
        .iter()
        .map(|s| {
            (
                s.first().unwrap().process_name.clone(),
                s.iter()
                    .fold(chrono::Duration::seconds(0), |acc, s| acc + s.duration),
            )
        })
        .map(|(name, d)| (name, DurationWrapper { duration: d }))
        .collect()
}

fn get_time_by_process(streaks: &[Streak]) -> Vec<(ProcessName, DurationWrapper)> {
    let grouped_streaks = group_streaks_by_process_name(streaks);

    aggregate_durations(&grouped_streaks)
}

impl TryFrom<Report> for Digest {
    type Error = ();

    fn try_from(report: Report) -> Result<Self, Self::Error> {
        if report.streaks.is_empty() {
            return Err(());
        }

        let (begin_date, end_date) = get_begin_and_end_dates(&report);
        let grouped_streaks = group_streaks_by_process_name(&report.streaks);
        let time_by_process = aggregate_durations(&grouped_streaks);
        let streak_data: Vec<StreakDigest> = grouped_streaks
            .into_iter()
            .map(StreakDigest::from)
            .collect();

        // Return value
        Ok(Digest {
            begin_date,
            end_date,
            time_by_process,
            streak_data,
            productivity_data: None,
        })
    }
}

#[cfg(test)]
mod tests {

    use std::collections::HashSet;

    use super::*;
    use crate::data::wrappers::ProcessName;

    fn build_streak(process_name: &str, duration: i64) -> Streak {
        // Let's say we dont care about the other parameters
        Streak {
            pid: 10,
            process_name: ProcessName(process_name.to_owned()),
            window_names: HashSet::default(),
            website_name: None,
            duration: chrono::Duration::seconds(duration),
            begin_date: chrono::DateTime::<chrono::Local>::default(),
        }
    }
    fn build_streak_list() -> Vec<Streak> {
        let process_name_1: String = "Toto".to_owned();
        let process_name_2: String = "Tata".to_owned();
        vec![
            build_streak(&process_name_1, 20),
            build_streak(&process_name_1, 30),
            build_streak(&process_name_1, 10),
            build_streak(&process_name_2, 20),
            build_streak(&process_name_2, 30),
            build_streak(&process_name_2, 40),
            build_streak(&process_name_2, 100),
        ]
    }

    #[test]
    fn test_group_streaks() {
        let streak_list = build_streak_list();
        let groups = group_streaks_by_process_name(&streak_list);
        assert_eq!(groups.len(), 2);
        assert_eq!(groups[0].len(), 3);
        assert_eq!(groups[1].len(), 4);
    }

    #[test]
    fn test_get_time_by_process() {
        let streak_list = build_streak_list();
        let durations = get_time_by_process(&streak_list);
        assert_eq!(
            durations[0],
            (
                ProcessName::from(String::from("Toto")),
                DurationWrapper {
                    duration: chrono::Duration::seconds(60)
                }
            )
        );
        assert_eq!(
            durations[1],
            (
                ProcessName::from(String::from("Tata")),
                DurationWrapper {
                    duration: chrono::Duration::seconds(190)
                }
            )
        );
    }
}
