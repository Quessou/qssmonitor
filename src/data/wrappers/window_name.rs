use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Default, sqlx::Type)]
#[sqlx(transparent)]
pub struct WindowName(pub String);

impl From<String> for WindowName {
    fn from(value: String) -> Self {
        WindowName(value)
    }
}
impl From<&str> for WindowName {
    fn from(value: &str) -> Self {
        WindowName(value.into())
    }
}
