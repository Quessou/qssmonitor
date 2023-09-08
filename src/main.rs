use config::Config;
use config::File;

mod aggregator;
mod data;
mod default_config;
mod filesystem;
mod logging;
mod process;
mod x;

use default_config::QssMonitorConfig;
use logging::initialization::initialize_subscriber;

use crate::aggregator::streak_extension_strategy::BrowserInclusiveStreakExtensionStrategy;

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

    let sample = data::SampleBuilder::default().build_sample();
    let mut aggregator = aggregator::Aggregator::new(
        chrono::Duration::seconds(5),
        Box::new(BrowserInclusiveStreakExtensionStrategy::default()),
    );
    println!("{}", sample);
    aggregator.register_sample(sample);
    Ok(())
}
