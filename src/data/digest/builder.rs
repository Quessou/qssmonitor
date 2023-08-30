use super::productivity_computation::ProductivityComputation;
use super::Digest;
use crate::data::report::Report;

pub struct Builder<Prod>
where
    Prod: ProductivityComputation,
{
    productivity_computation: Prod,
    unproductive_apps: Vec<String>,
}

impl<Prod> Builder<Prod>
where
    Prod: ProductivityComputation,
{
    pub fn new(unproductive_apps: Vec<String>) -> Self {
        Builder {
            productivity_computation: Prod::default(),
            unproductive_apps,
        }
    }

    pub fn build_digest(&self, report: Report) -> Digest {
        let productivity_data = self
            .productivity_computation
            .compute_productivity(&report, &self.unproductive_apps);
        let mut digest: Digest = report.try_into().unwrap();

        digest.productivity_data = Some(productivity_data);
        digest
    }
}
