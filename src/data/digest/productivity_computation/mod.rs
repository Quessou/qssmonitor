mod process_named_productivity_computation;

use crate::data::Report;
pub(crate) use process_named_productivity_computation::ProcessNamedProductivityComputation;

use super::ProductivityData;

pub trait ProductivityComputation: Default {
    fn compute_productivity(
        &self,
        report: &Report,
        non_productive_apps: &Vec<String>,
    ) -> ProductivityData;
}
