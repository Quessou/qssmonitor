use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqliteTypeInfo, Sqlite};
use std::str::FromStr;

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq, Eq, sqlx::Type)]
#[sqlx(transparent)]
pub struct WebsiteName(pub String);

impl WebsiteName {
    pub fn new(data: String) -> Self {
        Self { 0: data }
    }
}

impl From<String> for WebsiteName {
    fn from(value: String) -> Self {
        WebsiteName { 0: value }
    }
}
