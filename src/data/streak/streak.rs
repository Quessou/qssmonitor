use std::{assert_ne, collections::HashSet};

use crate::data::wrappers::{ProcessName, WebsiteName, WindowName};
use crate::data::Sample;
use chrono::Duration;

/// Set of samples that are contiguous in time, without the focus of the main window being changed
#[derive(Clone)]
pub struct Streak {
    pub pid: i32,
    pub process_name: ProcessName,
    pub window_names: HashSet<WindowName>,
    pub website_name: Option<WebsiteName>,
    pub duration: Duration,
    pub begin_date: chrono::DateTime<chrono::Local>,
}

impl From<(Vec<Sample>, Duration)> for Streak {
    fn from(value: (Vec<Sample>, Duration)) -> Self {
        let interval = value.1;
        let samples = value.0;
        assert_ne!(samples.len(), 0);
        let pid = samples[0].pid;
        let process_name = samples[0].process_name.clone();
        let duration = interval * (samples.len()).try_into().unwrap();
        let website_name = samples[0].website_name.clone();
        let window_names = samples
            .into_iter()
            .map(|sample| sample.window_name)
            .collect::<HashSet<WindowName>>();
        Streak {
            pid,
            process_name,
            window_names,
            website_name,
            duration,
            begin_date: chrono::Local::now(),
        }
    }
}
