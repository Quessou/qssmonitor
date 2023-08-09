use crate::filesystem::paths::*;
use serde::Serialize;

use configgen_rs::DefaultConfig;

pub fn initialize_configuration<T: DefaultConfig + Serialize + Default>(
    config: &T,
) -> Result<(), configgen_rs::Error> {
    configgen_rs::initialization::create_config_dir(get_config_dir_path())?;

    configgen_rs::initialization::initialize_config_file(
        &config,
        &get_config_file_path(),
        configgen_rs::SerializationFormat::Toml,
    )?;
    Ok(())
}
