use serde::de::Visitor;

use super::QssMontiorConfig;

pub struct DefaultConfigVisitor;

impl<'de> Visitor<'de> for DefaultConfigVisitor {
    type Value = QssMontiorConfig;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a map with keys 'first' and 'second'")
    }
}
