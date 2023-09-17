use std::error::Error;

use clap::arg;
use clap::ArgMatches;
use clap::Command;
use config::Config;
use config::File;

mod aggregator;
mod core;
mod data;
mod default_config;
mod endpoints;
mod filesystem;
mod logging;
mod messages;
mod process;
mod x;

use data::website_detection::DetectionData;

use data::SampleBuilder;
use default_config::QssMonitorConfig;
use logging::initialization::initialize_subscriber;

use crate::aggregator::streak_extension_strategy::BrowserInclusiveStreakExtensionStrategy;

use crate::core::Core;
use crate::data::website_detection::WebsiteNameDetector;

fn build_website_name_detector(non_productive_websites: Vec<DetectionData>) -> WebsiteNameDetector {
    WebsiteNameDetector::new(non_productive_websites)
}

fn build_sample_builder(non_productive_websites: Vec<DetectionData>) -> SampleBuilder {
    let website_name_detector = build_website_name_detector(non_productive_websites);
    data::SampleBuilder::new(
        x::Requester::default(),
        process::Requester::default(),
        website_name_detector,
    )
}

fn get_config() -> Result<QssMonitorConfig, Box<dyn Error>> {
    let config_file = File::with_name(filesystem::paths::get_config_file_path().to_str().unwrap());
    let default_config = QssMonitorConfig::default();

    if let Err(e) = filesystem::config_initialization::initialize_configuration(&default_config) {
        // TODO: Isn't there something better to do here ?
        if e != configgen_rs::Error::ConfigDirectoryAlreadyExists(std::io::Error::new(
            std::io::ErrorKind::AlreadyExists,
            "",
        )) {
            tracing::error!("Could not initialize configuration directory");
            return Err(Box::new(e));
        }
    };
    let loaded_config = Config::builder().add_source(config_file).build().unwrap();
    Ok(loaded_config.try_deserialize::<QssMonitorConfig>().unwrap())
}

fn get_args() -> ArgMatches {
    let command = Command::new("qssmonitor")
        .about(
            "Monitors the window in the foreground and computes statistics about your productivity",
        )
        .arg(arg!(--daemon "Launch the app in daemon mode"));
    command.get_matches()
}

#[tokio::main]
async fn main() {
    initialize_subscriber().unwrap();
    /*
    let command = Command::new("qssmonitor")
        .about(
            "Monitors the window in the foreground and computes statistics about your productivity",
        )
        .arg(arg!(--daemon "Launch the app in daemon mode"));
    let arguments = command.get_matches();
    if let Some(true) = arguments.get_one::<bool>("daemon") {
        println!("Daemon mode !!");
    }
    */
    let args = get_args();
    let read_config = match get_config() {
        Ok(c) => c,
        Err(e) => {
            tracing::error!("Could not read config : {:?}", e);
            panic!();
        }
    };

    let sample_builder = build_sample_builder(read_config.non_productive_website.clone());
    let aggregator = aggregator::Aggregator::new(
        // TODO : Replace by config value
        chrono::Duration::seconds(5),
        Box::new(BrowserInclusiveStreakExtensionStrategy::new()),
    );

    let core = Core::new(sample_builder, aggregator);
    let router = endpoints::generate_api(core.clone()).await;
    core.run(read_config, args, Some(router)).await.unwrap();
}
