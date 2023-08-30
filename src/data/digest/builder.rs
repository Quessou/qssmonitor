use super::productivity_computation::ProductivityComputation;
use super::{Digest, ProductivityData};
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
        //let productive_time =
        // TODO : check errors here
        let digest = report.try_into().unwrap();

        self.fill_productive_time(digest)
    }

    pub fn fill_productive_time(&self, digest: Digest) -> Digest {
        let productivity_data = ProductivityData {
            total_time: todo!(),
            productive_time: todo!(),
        };
        digest
    }
}
