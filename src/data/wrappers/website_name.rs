use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq, Eq, sqlx::Type)]
#[sqlx(transparent)]
pub struct WebsiteName(pub String);

impl WebsiteName {
    pub fn new(data: String) -> Self {
        Self(data)
    }
}

impl From<String> for WebsiteName {
    fn from(value: String) -> Self {
        WebsiteName(value)
    }
}
