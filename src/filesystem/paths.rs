use std::path::PathBuf;

use home::home_dir;

pub fn get_config_dir_path() -> PathBuf {
    home_dir().unwrap().join(".qssmonitor")
}

pub fn get_config_file_path() -> PathBuf {
    get_config_dir_path().join("config.toml")
}
