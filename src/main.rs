use config::Config;
use config::File;

mod aggregator;
mod data;
mod default_config;
mod filesystem;
mod logging;
mod process;
mod x;

use data::website_detection::WebsiteNameDetectionCriteria;
use data::wrappers::WebsiteName;
use data::SampleBuilder;
use default_config::QssMonitorConfig;
use logging::initialization::initialize_subscriber;

use crate::aggregator::streak_extension_strategy::BrowserInclusiveStreakExtensionStrategy;
use crate::data::website_detection::WebsiteNameDetector;

fn build_non_productive_websites_list() -> Vec<(WebsiteName, Vec<WebsiteNameDetectionCriteria>)> {
    use WebsiteNameDetectionCriteria as Criteria;
    vec![
        (
            WebsiteName {
                0: "Instagram".to_owned(),
            },
            vec![
                Criteria::EndsWith("· Direct".to_owned()),
                Criteria::EndsWith("Instagram".to_owned()),
                Criteria::EndsWith("Instagram photos and videos".to_owned()),
            ],
        ),
        (
            WebsiteName {
                0: "Twitter".to_owned(),
            },
            vec![
                Criteria::EndsWith("/ Twitter".to_owned()),
                Criteria::EndsWith("/ X".to_owned()),
            ],
        ),
        (
            WebsiteName {
                0: "Whatsapp".to_owned(),
            },
            vec![
                Criteria::EndsWith("Whatsapp Web".to_owned()),
                Criteria::EndsWith("Whatsapp".to_owned()),
            ],
        ),
        (
            WebsiteName {
                0: "Mediapart".to_owned(),
            },
            vec![
                Criteria::Contains("Mediapart".to_owned()),
                Criteria::EndsWith("| Mediapart".to_owned()),
            ],
        ),
        (
            WebsiteName {
                0: "Linkedin".to_owned(),
            },
            vec![Criteria::EndsWith("LinkedIn".to_owned())],
        ),
        (
            WebsiteName {
                0: "Facebook".to_owned(),
            },
            vec![Criteria::EndsWith("Facebook".to_owned())],
        ),
        (
            WebsiteName {
                0: "Netflix".to_owned(),
            },
            vec![Criteria::EndsWith("Netflix".to_owned())],
        ),
        (
            WebsiteName {
                0: "Youtube".to_owned(),
            },
            vec![Criteria::EndsWith("YouTube".to_owned())],
        ),
        (
            WebsiteName {
                0: "OkCupid".to_owned(),
            },
            vec![Criteria::EndsWith("OkCupid".to_owned())],
        ),
        (
            WebsiteName {
                0: "Tinder".to_owned(),
            },
            vec![Criteria::Contains("Tinder".to_owned())],
        ),
        (
            WebsiteName {
                0: "Gmail".to_owned(),
            },
            vec![Criteria::EndsWith("Gmail".to_owned())],
        ),
        (
            WebsiteName {
                0: "Scryfall".to_owned(),
            },
            vec![Criteria::Contains("Scryfall".to_owned())],
        ),
        (
            WebsiteName {
                0: "EDHREC".to_owned(),
            },
            vec![Criteria::EndsWith("EDHREC".to_owned())],
        ),
        (
            WebsiteName {
                0: "Moxfield".to_owned(),
            },
            vec![Criteria::Contains("// Moxfield".to_owned())],
        ),
        (
            WebsiteName {
                0: "Amazon".to_owned(),
            },
            vec![Criteria::Contains("Amazon".to_owned())],
        ),
        (
            WebsiteName {
                0: "Twitch".to_owned(),
            },
            vec![Criteria::EndsWith("Twitch".to_owned())],
        ),
    ]
}

fn build_website_name_detector(
    non_productive_websites: Vec<(WebsiteName, Vec<WebsiteNameDetectionCriteria>)>,
) -> WebsiteNameDetector {
    WebsiteNameDetector::new(non_productive_websites)
}

fn build_sample_builder() -> SampleBuilder {
    let non_productive_websites = build_non_productive_websites_list();
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

    let sample_builder = build_sample_builder();
    let sample = sample_builder.build_sample();
    let mut aggregator = aggregator::Aggregator::new(
        chrono::Duration::seconds(5),
        Box::new(BrowserInclusiveStreakExtensionStrategy::new()),
    );
    println!("{}", sample);
    aggregator.register_sample(sample);
    Ok(())
}
