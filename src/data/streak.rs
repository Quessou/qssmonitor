use std::{assert_ne, collections::HashSet};

use crate::data::Sample;
use crate::{process::ProcessName, x::WindowName};
use chrono::Duration;

#[derive(Clone)]
pub struct Streak {
    pid: i32,
    process_name: ProcessName,
    window_names: HashSet<WindowName>,
    duration: Duration,
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
        }
    }
}
