use chrono::DateTime;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

use crate::data::wrappers::{ProcessName, WindowName};

use super::wrappers::WebsiteName;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Sample {
    pub timestamp: DateTime<chrono::Local>,
    pub process_name: ProcessName,
    pub window_name: WindowName,
    pub website_name: Option<WebsiteName>,
    pub pid: i32,
}

impl Sample {
    pub fn new(
        process_name: ProcessName,
        window_name: WindowName,
        website_name: Option<WebsiteName>,
        pid: i32,
    ) -> Self {
        Sample {
            timestamp: chrono::Local::now(),
            process_name,
            window_name,
            website_name,
            pid,
        }
    }
}

impl From<(String, String, String, i32)> for Sample {
    fn from(value: (String, String, String, i32)) -> Self {
        let website_name: Option<WebsiteName> = if value.2.is_empty() {
            None
        } else {
            Some(value.2.into())
        };
        Sample::new(value.0.into(), value.1.into(), website_name, value.3)
    }
}
impl From<(ProcessName, WindowName, Option<WebsiteName>, i32)> for Sample {
    fn from(value: (ProcessName, WindowName, Option<WebsiteName>, i32)) -> Self {
        Sample::new(value.0, value.1, value.2, value.3)
    }
}

impl Display for Sample {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Timestamp: {},\nProcess name : {},\nWindow name : {},\nWebsite name : {}\nPID : {}\n",
            self.timestamp,
            self.process_name.0,
            self.window_name.0,
            self.website_name
                .as_ref()
                .unwrap_or(&WebsiteName {
                    data: "Productive I guess ?".to_owned()
                })
                .data,
            self.pid
        )
    }
}
