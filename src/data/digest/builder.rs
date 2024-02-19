use super::productivity_computation::{self, ProductivityComputation};
use super::Digest;
use crate::data::report::Report;
use crate::data::website_detection::DetectionData;
use crate::data::wrappers::website_name::WebsiteName;

#[derive(Debug, Clone)]
pub struct Builder<Prod>
where
    Prod: ProductivityComputation,
{
    productivity_computation: Prod,
}

impl<Prod> Builder<Prod>
where
    Prod: ProductivityComputation,
{
    pub fn new(productivity_computation: Prod) -> Self {
        //unproductive_apps: Vec<String>, unproductive_websites: Vec<DetectionData>) -> Self {
        Builder {
            productivity_computation, //      unproductive_apps,
                                      //       unproductive_websites,
        }
    }

    pub fn build_digest(&self, report: Report) -> Digest {
        let productivity_data = self.productivity_computation.compute_productivity(&report); //, &self.unproductive_apps);
        let mut digest: Digest = report.try_into().unwrap();

        digest.productivity_data = Some(productivity_data);
        digest
    }
}
