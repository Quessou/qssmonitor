use chrono::DateTime;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

use crate::process::ProcessName;
use crate::x::WindowName;

#[derive(Debug, Serialize, Deserialize)]
pub struct Sample {
    timestamp: DateTime<chrono::Utc>,
    process_name: ProcessName,
    window_name: WindowName,
    pid: i32,
}

impl Sample {
    pub fn new(process_name: ProcessName, window_name: WindowName, pid: i32) -> Self {
        Sample {
            timestamp: chrono::offset::Utc::now(),
            process_name,
            window_name,
            pid,
        }
    }
}

impl From<(String, String, i32)> for Sample {
    fn from(value: (String, String, i32)) -> Self {
        Sample::new(value.0.into(), value.1.into(), value.2)
    }
}
impl From<(ProcessName, WindowName, i32)> for Sample {
    fn from(value: (ProcessName, WindowName, i32)) -> Self {
        Sample::new(value.0, value.1, value.2)
    }
}

impl Display for Sample {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Timestamp: {},\nProcess name : {},\nWindow name : {},\nPID : {}\n",
            self.timestamp, self.process_name.0, self.window_name.0, self.pid
        )
    }
}
