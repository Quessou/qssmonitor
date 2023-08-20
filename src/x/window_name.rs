use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct WindowName(pub String);

impl From<String> for WindowName {
    fn from(value: String) -> Self {
        WindowName(value)
    }
}
