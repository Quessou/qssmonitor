use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString};

#[derive(Serialize, Deserialize, Display, EnumIter, EnumString, Debug, Clone)]
#[strum(serialize_all = "snake_case")]
pub enum DetectionDiscriminant {
    StartsWith,
    Contains,
    EndsWith,
}
