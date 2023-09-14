use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WebsiteName {
    pub data: String,
}
impl From<String> for WebsiteName {
    fn from(value: String) -> Self {
        WebsiteName { data: value }
    }
}
