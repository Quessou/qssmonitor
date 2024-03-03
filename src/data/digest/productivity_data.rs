use crate::data::wrappers::DurationWrapper;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct ProductivityData {
    /// Productive time + Pause time + Unproductive time
    pub total_time: DurationWrapper,
    /// Productive time (without pauses)
    pub productive_time: DurationWrapper,
    /// Pause time
    pub pause_time: DurationWrapper,
}
