#[derive(
    Hash, Eq, PartialOrd, PartialEq, Clone, serde::Serialize, serde::Deserialize, Debug, sqlx::Type,
)]
#[sqlx(transparent)]
pub struct ProcessName(pub String);

impl From<String> for ProcessName {
    fn from(value: String) -> Self {
        ProcessName(value)
    }
}
impl From<&str> for ProcessName {
    fn from(value: &str) -> Self {
        ProcessName(value.into())
    }
}

impl Ord for ProcessName {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}
