pub(crate) mod db_types;
pub(crate) mod digest;
pub(crate) mod report;
pub(crate) mod sample;
pub(crate) mod sample_builder;
pub(crate) mod streak;
pub(crate) mod website_detection;
pub(crate) mod wrappers;

pub use report::Report;
pub use sample::Sample;
pub use sample_builder::SampleBuilder;
pub use streak::Streak;

use crate::process::Requester as ProcessRequester;
use crate::x::Requester as XRequester;

use self::website_detection::{build_website_name_detector, DetectionData};

pub fn build_sample_builder(non_productive_websites: Vec<DetectionData>) -> SampleBuilder {
    let website_name_detector = build_website_name_detector(non_productive_websites);
    SampleBuilder::new(
        XRequester::default(),
        ProcessRequester::default(),
        website_name_detector,
    )
}
