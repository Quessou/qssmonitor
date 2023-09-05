use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct WebsiteName(pub String);
impl From<String> for WebsiteName {
    fn from(value: String) -> Self {
        WebsiteName(value)
    }
}
