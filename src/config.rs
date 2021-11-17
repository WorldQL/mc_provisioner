use std::{collections::BTreeMap, path::PathBuf};

use color_eyre::Result;
use derive_getters::Getters;
use serde::Deserialize;

// region: TOML
#[derive(Debug, Default, Deserialize, Getters)]
pub struct Config {
    global: Option<GlobalConfig>,
    init: Option<InitConfig>,
}

#[derive(Debug, Default, Deserialize, Getters)]
pub struct GlobalConfig {
    server_count: Option<u8>,
    start_port: Option<u16>,
    directory_template: Option<String>,
}

#[derive(Debug, Default, Deserialize, Getters)]
pub struct InitConfig {
    paper_version: Option<String>,
    level_seed: Option<String>,
    skip_plugins: Option<bool>,
    no_copy_bukkit: Option<bool>,
    no_copy_spigot: Option<bool>,
    no_copy_paper: Option<bool>,
    server_properties: Option<BTreeMap<String, String>>,
}

#[derive(Debug, Default, Deserialize, Getters)]
pub struct StartConfig {
    max_memory: Option<String>,
}

pub fn read_config() -> Result<Config> {
    let path = PathBuf::from("provisioner.toml");
    if !path.exists() || !path.is_file() {
        return Ok(Default::default())
    }

    let file = std::fs::read(&path)?;
    let config = toml::from_slice::<Config>(&file)?;

    Ok(config)
}
// endregion
