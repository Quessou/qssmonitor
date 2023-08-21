use super::report::Report;

pub struct Digest {}

impl From<Report> for Digest {
    fn from(_value: Report) -> Self {
        todo!()
    }
}
