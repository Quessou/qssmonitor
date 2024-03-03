use axum::Router;
use clap::ArgMatches;

use futures::StreamExt;
use signal_hook::consts::signal::*;
use signal_hook_tokio::Signals;
use std::{borrow::BorrowMut, sync::Arc};
use tokio::{
    sync::mpsc::channel,
    sync::Mutex,
    task::{self},
};
use tracing::{self, instrument, Instrument};

use crate::{
    aggregator::Aggregator,
    data::{
        digest::{Builder as DigestBuilder, ProductivityComputation},
        Report, Sample, SampleBuilder,
    },
    database::DatabaseAccess,
    default_config::QssMonitorConfig,
    messages::QssMonitorMessage,
};

#[derive(Clone, Debug)]
pub struct Core<
    DB: DatabaseAccess + std::fmt::Debug,
    Prod: ProductivityComputation + std::fmt::Debug,
> {
    sample_builder: Arc<Mutex<SampleBuilder>>,
    pub aggregator: Arc<Mutex<Aggregator<DB>>>,
    pub digest_builder: Arc<Mutex<DigestBuilder<Prod>>>,
    is_paused: Arc<Mutex<bool>>,
}

impl<
        DB: DatabaseAccess + std::fmt::Debug + std::marker::Sync + 'static,
        PC: ProductivityComputation + std::fmt::Debug + 'static,
    > Core<DB, PC>
{
    pub async fn get_last_report(&self) -> Report {
        self.aggregator.lock().await.get_current_report()
    }

    pub fn new(
        sample_builder: SampleBuilder,
        aggregator: Aggregator<DB>,
        digest_builder: DigestBuilder<PC>,
    ) -> Self {
        Core {
            sample_builder: Arc::new(Mutex::new(sample_builder)),
            aggregator: Arc::new(Mutex::new(aggregator)),
            digest_builder: Arc::new(Mutex::new(digest_builder)),
            is_paused: Arc::new(Mutex::new(false)),
        }
    }

    #[instrument]
    pub async fn run(
        &self,
        config: QssMonitorConfig,
        args: ArgMatches,
        router: Option<Router>,
    ) -> Result<(), ()> {
        let (sampling_sender, mut sampling_receiver) = channel::<QssMonitorMessage>(5);
        let core_clone = self.clone();
        let sampling_task = task::spawn(async move {
            if core_clone
                .aggregator
                .lock()
                .await
                .start_session()
                .await
                .is_err()
            {
                tracing::error!("Session creation in DB failed. Panicking.");
                panic!();
            }
            let mut interval = tokio::time::interval(config.polling_interval.to_std().unwrap());
            while let Err(tokio::sync::mpsc::error::TryRecvError::Empty) =
                sampling_receiver.try_recv()
            {
                interval.tick().await;
                let sample = if *core_clone.is_paused.lock().await {
                    Some(Sample::build_pause_sample())
                } else {
                    core_clone.sample_builder.lock().await.build_sample().await
                };
                if sample.is_none() {
                    tracing::warn!("Could not build sample, skipping");
                    continue;
                }
                core_clone
                    .aggregator
                    .lock()
                    .await
                    .register_sample(sample.unwrap())
                    .await;
            }
            tracing::info!("Stopping sampling");
        })
        .instrument(tracing::error_span!("Sampling"));

        let (serving_sender, mut serving_receiver) = channel::<QssMonitorMessage>(5);
        let serving_task = task::spawn(async move {
            axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
                .serve(router.unwrap().into_make_service())
                .with_graceful_shutdown(async {
                    serving_receiver.recv().await;
                })
                .await
                .unwrap();
            tracing::info!("Stopping webserver");
        })
        .instrument(tracing::error_span!("Web server"));

        let signal_polling_task = task::spawn(async move {
            let mut signals =
                Signals::new([SIGHUP, SIGTERM, SIGINT, SIGQUIT, SIGABRT, SIGTSTP]).unwrap();
            let handle = signals.handle();
            while let Some(signal) = signals.next().await {
                match signal {
                    SIGTERM | SIGINT | SIGQUIT => {
                        sampling_sender.send(QssMonitorMessage::Stop).await.unwrap();
                        serving_sender.send(QssMonitorMessage::Stop).await.unwrap();
                        handle.close();
                    }
                    SIGHUP => {
                        sampling_sender.send(QssMonitorMessage::Stop).await.unwrap();
                        serving_sender.send(QssMonitorMessage::Stop).await.unwrap();
                        handle.close();
                    }
                    SIGABRT => {
                        sampling_sender.send(QssMonitorMessage::Stop).await.unwrap();
                        serving_sender.send(QssMonitorMessage::Stop).await.unwrap();
                        handle.close();
                    }
                    SIGTSTP => {
                        sampling_sender.send(QssMonitorMessage::Stop).await.unwrap();
                        serving_sender.send(QssMonitorMessage::Stop).await.unwrap();
                        handle.close();
                    }
                    _ => {
                        tracing::debug!("Not a signal we care about")
                    }
                }
            }
        });

        let _toto = futures::join!(sampling_task, serving_task, signal_polling_task);

        Ok(())
    }

    pub async fn toggle_pause(&mut self) {
        let mut is_paused = self.is_paused.lock().await;
        *is_paused = !*is_paused;
    }
    pub async fn is_paused(&mut self) -> bool {
        return *self.is_paused.lock().await;
    }
}
