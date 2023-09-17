use axum::{Router};
use clap::ArgMatches;

use std::sync::Arc;
use std::time::Duration;
use tokio::{
    sync::Mutex,
    task::{self},
};
use tracing::{self, instrument, Instrument};

use crate::{
    aggregator::Aggregator,
    data::{
        self,
        website_detection::{DetectionData, WebsiteNameDetector},
        Report, SampleBuilder,
    },
    default_config::QssMonitorConfig, process, x,
};

#[derive(Clone, Debug)]
pub struct Core {
    sample_builder: Arc<Mutex<SampleBuilder>>,
    pub aggregator: Arc<Mutex<Aggregator>>,
}

impl Core {
    fn build_website_name_detector(
        non_productive_websites: Vec<DetectionData>,
    ) -> WebsiteNameDetector {
        WebsiteNameDetector::new(non_productive_websites)
    }

    fn build_sample_builder(non_productive_websites: Vec<DetectionData>) -> SampleBuilder {
        let website_name_detector = Self::build_website_name_detector(non_productive_websites);
        data::SampleBuilder::new(
            x::Requester::default(),
            process::Requester::default(),
            website_name_detector,
        )
    }

    async fn get_last_report(&self) -> Report {
        self.aggregator.lock().await.get_report()
    }

    pub fn new(sample_builder: SampleBuilder, aggregator: Aggregator) -> Self {
        Core {
            sample_builder: Arc::new(Mutex::new(sample_builder)),
            aggregator: Arc::new(Mutex::new(aggregator)),
            //router: None,
        }
    }

    #[instrument]
    pub async fn run(
        &self,
        config: QssMonitorConfig,
        args: ArgMatches,
        router: Option<Router>,
    ) -> Result<(), ()> {
        let clone = self.clone();
        let sampling_task = task::spawn(async move {
            tracing::error!("log at the beginning of the async move block");
            let mut interval = tokio::time::interval(Duration::new(1, 0));
            loop {
                interval.tick().await;
                let sample = clone.sample_builder.lock().await.build_sample().await;
                clone.aggregator.lock().await.register_sample(sample);
            }
        })
        .instrument(tracing::error_span!("Sampling"));

        let serving_task = task::spawn(async move {
            axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
                .serve(router.unwrap().into_make_service())
                .await
                .unwrap();
        })
        .instrument(tracing::error_span!("Web server"));

        let _toto = futures::join!(sampling_task, serving_task);

        Ok(())
    }
}
