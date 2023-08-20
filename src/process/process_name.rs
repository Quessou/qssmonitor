

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ProcessName(pub String);

impl From<String> for ProcessName {
    fn from(value: String) -> Self {
        ProcessName(value)
    }
}
