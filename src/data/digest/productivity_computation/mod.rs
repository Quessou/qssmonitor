mod complete_productivity_computation;
mod process_named_productivity_computation;

use crate::data::Report;

use super::ProductivityData;

pub use complete_productivity_computation::CompleteProductivityComputation;

pub trait ProductivityComputation:
    Default + Clone + std::fmt::Debug + std::marker::Sync + std::marker::Send
{
    fn compute_productivity(&self, report: &Report) -> ProductivityData;
}
