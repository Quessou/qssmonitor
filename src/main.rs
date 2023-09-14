


use config::Config;
use config::File;

mod aggregator;
mod data;
mod default_config;
mod filesystem;
mod logging;
mod process;
mod x;


use data::website_detection::DetectionData;

use data::SampleBuilder;
use default_config::QssMonitorConfig;
use logging::initialization::initialize_subscriber;

use crate::aggregator::streak_extension_strategy::BrowserInclusiveStreakExtensionStrategy;

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config_file = File::with_name(filesystem::paths::get_config_file_path().to_str().unwrap());
    let config = QssMonitorConfig::default();

    let _logging_guard = initialize_subscriber();

    if let Err(e) = filesystem::config_initialization::initialize_configuration(&config) {
        // Isn't there something better to do here ?
        if e != configgen_rs::Error::ConfigDirectoryAlreadyExists(std::io::Error::new(
            std::io::ErrorKind::AlreadyExists,
            "",
        )) {
            tracing::error!("Could not initialize configuration directory");
            return Err(Box::new(e));
        }
    };
    let loaded_config = Config::builder().add_source(config_file).build().unwrap();
    let _read_config = loaded_config.try_deserialize::<QssMonitorConfig>().unwrap();

    /*
    let toto = build_non_productive_websites_list();
    let toto = toto
        .into_iter()
        .map(|t| t.into())
        .collect::<Vec<DetectionData>>();
    #[derive(serde::Serialize, serde::Deserialize)]
    struct Gngn {
        toto: Vec<DetectionData>,
    }
    let gngn = Gngn { toto };
    match toml::to_string(&gngn) {
        Ok(s) => println!("{}", s),
        Err(e) => println!("{:?} {}", e, e),
    }
    */

    let sample_builder = build_sample_builder(config.non_productive_website.clone());
    let sample = sample_builder.build_sample();
    let mut aggregator = aggregator::Aggregator::new(
        chrono::Duration::seconds(5),
        Box::new(BrowserInclusiveStreakExtensionStrategy::new()),
    );
    println!("{}", sample);
    aggregator.register_sample(sample);
    Ok(())
}
