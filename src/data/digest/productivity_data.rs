use crate::data::wrappers::DurationWrapper;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct ProductivityData {
    pub total_time: DurationWrapper,
    pub productive_time: DurationWrapper,
}
