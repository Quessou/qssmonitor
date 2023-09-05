mod digest;

use std::{assert_ne, collections::HashSet};

use crate::data::wrappers::{ProcessName, WindowName};
use crate::data::Sample;
use chrono::Duration;
pub use digest::StreakDigest;

/// Set of samples that are contiguous in time, without the focus of the main window being changed
#[derive(Clone)]
pub struct Streak {
    pub pid: i32,
    pub process_name: ProcessName,
    pub window_names: HashSet<WindowName>,
    pub duration: Duration,
    pub begin_date: chrono::DateTime<chrono::Local>,
}

impl From<(Vec<Sample>, Duration)> for Streak {
    fn from(value: (Vec<Sample>, Duration)) -> Self {
        let interval = value.1;
        let value = value.0;
        assert_ne!(value.len(), 0);
        let pid = value[0].pid;
        let process_name = value[0].process_name.clone();
        let duration = interval * (value.len()).try_into().unwrap();
        let window_names = value
            .into_iter()
            .map(|sample| sample.window_name)
            .collect::<HashSet<WindowName>>();
        Streak {
            pid,
            process_name,
            window_names,
            duration,
            begin_date: chrono::Local::now(),
        }
    }
}
