use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Default, sqlx::Type)]
#[sqlx(transparent)]
pub struct WindowName(pub String);

impl From<String> for WindowName {
    fn from(value: String) -> Self {
        WindowName(value)
    }
}
