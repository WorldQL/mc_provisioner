use std::collections::{BTreeMap, HashSet};
use std::path::PathBuf;

use color_eyre::Result;
use serde::Deserialize;

use crate::arg_types::{self, JarType, ServerMemory, ServerProperty};
use crate::Args;

// region: TOML
#[derive(Debug, Default, Deserialize)]
pub struct Config {
    pub global: Option<GlobalConfig>,
    pub init: Option<InitConfig>,
    pub start: Option<StartConfig>,
    pub world_management: Option<WorldManagementConfig>,
}

#[derive(Debug, Default, Deserialize)]
pub struct GlobalConfig {
    jar_type: Option<JarType>,
    jar_version: Option<String>,
    server_count: Option<u8>,
    start_port: Option<u16>,
    level_name: Option<String>,
    directory_template: Option<String>,
    sync_dirs: Option<Vec<PathBuf>>,
    timeout_secs: Option<u8>,
}

#[derive(Debug, Default, Deserialize)]
pub struct InitConfig {
    level_seed: Option<String>,
    ops: Option<Vec<String>>,
    white_list: Option<Vec<String>>,
    server_properties: Option<BTreeMap<String, String>>,
}

#[derive(Debug, Default, Deserialize)]
pub struct StartConfig {
    max_memory: Option<ServerMemory>,
    use_aikar_flags: Option<bool>,
    jvm_args: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
pub struct WorldManagementConfig {
    pub world_diameter: Option<u32>,
    pub slice_width: Option<u32>,
    pub avoid_slicing_origin: Option<bool>,
    pub origin_radius: Option<u32>,
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
    pub jar_type: JarType,
    pub jar_version: String,
    pub server_count: u8,
    pub start_port: u16,
    pub level_name: String,
    pub directory_template: String,
    pub sync_dirs: Vec<PathBuf>,
    pub timeout_secs: u8,
}

pub fn global_args(config: GlobalConfig, args: Args) -> GlobalArgs {
    let sync_dirs = if args.sync_dirs.is_empty() {
        config
            .sync_dirs
            .unwrap_or_else(|| vec![PathBuf::from("./plugins")])
    } else {
        args.sync_dirs
    };

    GlobalArgs {
        jar_type: args.jar_type.or(config.jar_type).unwrap_or_default(),
        jar_version: args.jar_version.or(config.jar_version).unwrap_or_default(),
        server_count: args.server_count.or(config.server_count).unwrap_or(2),
        start_port: args.start_port.or(config.start_port).unwrap_or(25565),
        level_name: args
            .level_name
            .or(config.level_name)
            .unwrap_or("world".into()),
        directory_template: args
            .directory_template
            .or(config.directory_template)
            .unwrap_or_else(|| "Mammoth Server".into()),
        sync_dirs,
        timeout_secs: args.timeout_secs.or(config.timeout_secs).unwrap_or(10),
    }
}

#[derive(Debug)]
pub struct InitArgs {
    pub level_seed: String,
    pub ops: HashSet<String>,
    pub white_list: HashSet<String>,
    pub server_properties: Vec<ServerProperty>,
}

#[allow(clippy::too_many_arguments)]
pub fn init_args(
    config: InitConfig,
    level_seed: Option<String>,
    mut ops: Vec<String>,
    mut white_list: Vec<String>,
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
        level_seed: level_seed.or(config.level_seed).unwrap_or_default(),
        ops,
        white_list,
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

pub struct WorldManagementArgs {
    pub world_diameter: Option<u32>,
    pub slice_width: Option<u32>,
    pub avoid_slicing_origin: Option<bool>,
    pub origin_radius: Option<u32>,
}

pub fn world_management_args(
    config: WorldManagementConfig,
    world_diameter: Option<u32>,
    slice_width: Option<u32>,
    avoid_slicing_origin: Option<bool>,
    origin_radius: Option<u32>,
) -> WorldManagementArgs {
    WorldManagementArgs {
        world_diameter: world_diameter.or(config.world_diameter),
        slice_width: slice_width.or(config.slice_width),
        avoid_slicing_origin: avoid_slicing_origin.or(config.avoid_slicing_origin),
        origin_radius: origin_radius.or(config.origin_radius),
    }
}
// endregion
