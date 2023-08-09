use config::Config;
use config::File;

mod default_config;
mod filesystem;

use default_config::QssMontiorConfig;

fn main() {
    let config_file = File::with_name(filesystem::paths::get_config_file_path().to_str().unwrap());
    let config = QssMontiorConfig::default(); /*Config::builder()
                                              .add_source(config_file)
                                              .set_default("polling_interval_s", 5)
                                              .unwrap()
                                              .build()
                                              .unwrap();*/

    let r = filesystem::config_initialization::initialize_configuration(&config);
    let loaded_config = Config::builder()
        .add_source(config::File::from(config_file))
        .build()
        .unwrap();
    let read_config = loaded_config.try_deserialize::<QssMontiorConfig>().unwrap();

    println!("{:?}", read_config)
}
