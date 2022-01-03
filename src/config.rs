use std::collections::{BTreeMap, HashSet};
use std::path::PathBuf;

use color_eyre::Result;
use serde::Deserialize;

use crate::arg_types::{self, ServerMemory, ServerProperty};
use crate::{utils, Args};

// region: TOML
#[derive(Debug, Default, Deserialize)]
pub struct Config {
    pub global: Option<GlobalConfig>,
    pub init: Option<InitConfig>,
    pub start: Option<StartConfig>,
}

#[derive(Debug, Default, Deserialize)]
pub struct GlobalConfig {
    server_count: Option<u8>,
    start_port: Option<u16>,
    directory_template: Option<String>,
    timeout_secs: Option<u8>,
}

#[derive(Debug, Default, Deserialize)]
pub struct InitConfig {
    paper_version: Option<String>,
    level_seed: Option<String>,
    ops: Option<Vec<String>>,
    white_list: Option<Vec<String>>,
    skip_plugins: Option<bool>,
    no_copy_bukkit: Option<bool>,
    no_copy_spigot: Option<bool>,
    no_copy_paper: Option<bool>,
    server_properties: Option<BTreeMap<String, String>>,
}

#[derive(Debug, Default, Deserialize)]
pub struct StartConfig {
    max_memory: Option<ServerMemory>,
    use_aikar_flags: Option<bool>,
    jvm_args: Option<String>,
}

pub fn read_config() -> Result<Config> {
    let path = PathBuf::from("provisioner.toml");
    if !path.exists() || !path.is_file() {
        return Ok(Default::default());
    }

    let file = std::fs::read(&path)?;
    let config = toml::from_slice::<Config>(&file)?;

    Ok(config)
}
// endregion

// region: Merge Config
#[derive(Debug)]
pub struct GlobalArgs {
    pub server_count: u8,
    pub start_port: u16,
    pub directory_template: String,
    pub timeout_secs: u8,
}

pub fn global_args(config: GlobalConfig, args: Args) -> GlobalArgs {
    GlobalArgs {
        server_count: args.server_count.or(config.server_count).unwrap_or(2),
        start_port: args.start_port.or(config.start_port).unwrap_or(25565),
        directory_template: args
            .directory_template
            .or(config.directory_template)
            .unwrap_or_else(|| "Mammoth Server".into()),
        timeout_secs: args.timeout_secs.or(config.timeout_secs).unwrap_or(10),
    }
}

#[derive(Debug)]
pub struct InitArgs {
    pub paper_version: String,
    pub level_seed: String,
    pub ops: HashSet<String>,
    pub white_list: HashSet<String>,
    pub skip_plugins: bool,
    pub no_copy_bukkit: bool,
    pub no_copy_spigot: bool,
    pub no_copy_paper: bool,
    pub server_properties: Vec<ServerProperty>,
}

#[allow(clippy::too_many_arguments)]
pub fn init_args(
    config: InitConfig,
    paper_version: Option<String>,
    level_seed: Option<String>,
    mut ops: Vec<String>,
    mut white_list: Vec<String>,
    skip_plugins: Option<bool>,
    no_copy_bukkit: Option<bool>,
    no_copy_spigot: Option<bool>,
    no_copy_paper: Option<bool>,
    server_properties: Vec<ServerProperty>,
) -> Result<InitArgs> {
    let ops = {
        let mut config_ops = config.ops.unwrap_or_default();
        config_ops.append(&mut ops);

        HashSet::from_iter(config_ops.into_iter())
    };

    let white_list = {
        let mut config_white_list = config.white_list.unwrap_or_default();
        config_white_list.append(&mut white_list);

        HashSet::from_iter(config_white_list.into_iter())
    };

    let server_properties = {
        let mut arg_props = arg_types::properties_to_map(server_properties);
        let mut config_props = config
            .server_properties
            .map(|props| {
                props
                    .into_iter()
                    .map(|(key, value)| (key.to_lowercase(), value))
                    .collect::<BTreeMap<_, _>>()
            })
            .unwrap_or_default();

        config_props.append(&mut arg_props);

        // Set white list to true
        if !white_list.is_empty() {
            config_props.insert("white-list".into(), "true".into());
        }

        config_props
    };

    let args = InitArgs {
        paper_version: paper_version
            .or(config.paper_version)
            .unwrap_or_else(|| "1.17.1".into()),
        level_seed: level_seed.or(config.level_seed).unwrap_or_default(),
        ops,
        white_list,
        skip_plugins: skip_plugins.or(config.skip_plugins).unwrap_or_default(),
        no_copy_bukkit: no_copy_bukkit.or(config.no_copy_bukkit).unwrap_or_default(),
        no_copy_spigot: no_copy_spigot.or(config.no_copy_spigot).unwrap_or_default(),
        no_copy_paper: no_copy_paper.or(config.no_copy_paper).unwrap_or_default(),
        server_properties: arg_types::map_to_properties(server_properties)?,
    };

    Ok(args)
}

#[derive(Debug)]
pub struct StartArgs {
    pub max_memory: ServerMemory,
    pub use_aikar_flags: bool,
    pub jvm_args: Option<String>,
}

pub fn start_args(
    config: StartConfig,
    max_memory: Option<ServerMemory>,
    use_aikar_flags: Option<bool>,
    jvm_args: Option<String>,
) -> StartArgs {
    StartArgs {
        max_memory: max_memory
            .or(config.max_memory)
            .unwrap_or_else(|| "1G".into()),
        use_aikar_flags: use_aikar_flags.or(config.use_aikar_flags).unwrap_or(false),
        jvm_args: jvm_args.or(config.jvm_args),
    }
}
// endregion
