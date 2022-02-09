#![forbid(unsafe_code)]
#![deny(private_in_public)]
#![warn(
    clippy::all,
    clippy::dbg_macro,
    clippy::todo,
    clippy::empty_enum,
    clippy::enum_glob_use,
    clippy::unused_self,
    clippy::needless_continue,
    clippy::needless_borrow,
    clippy::match_wildcard_for_single_variants,
    clippy::if_let_mutex,
    clippy::mismatched_target_os,
    clippy::match_on_vec_items,
    clippy::imprecise_flops,
    clippy::suboptimal_flops,
    clippy::lossy_float_literal,
    clippy::fn_params_excessive_bools,
    clippy::inefficient_to_string,
    clippy::macro_use_imports,
    clippy::option_option,
    clippy::unnested_or_patterns,
    clippy::str_to_string,
    clippy::cast_lossless,
    clippy::implicit_clone,
    clippy::unused_async,
    clippy::redundant_closure_for_method_calls,
    rust_2018_idioms,
    future_incompatible,
    nonstandard_style,
    missing_debug_implementations
)]

use std::path::PathBuf;

use arg_types::{JarType, ServerMemory, ServerProperty};
use clap::{IntoApp, Parser, ValueHint};
use clap_complete::Shell;
use color_eyre::Result;
use tracing::{error, warn};

mod arg_types;
mod cmd_combine_prune;
mod cmd_init;
mod cmd_remove;
mod cmd_reset_world;
mod cmd_start_stop;
mod cmd_sync;
mod cmd_update_server;
mod config;
mod server_jar;
mod utils;

#[derive(Debug, Clone, Parser)]
#[clap(about, version)]
pub struct Args {
    /// Server .jar type [default: "paper"]
    #[clap(short, long, value_hint = ValueHint::Other)]
    jar_type: Option<JarType>,

    /// Server .jar version
    #[clap(short = 'J', long, value_hint = ValueHint::Other)]
    jar_version: Option<String>,

    /// Number of servers to initialise [default: 2]
    #[clap(short = 'c', long, value_hint = ValueHint::Other)]
    server_count: Option<u8>,

    /// Server port to start counting at [default: 25565]
    #[clap(short = 'p', long, value_hint = ValueHint::Other)]
    start_port: Option<u16>,

    /// Level name [default: "world"]
    #[clap(short = 'w', long, value_hint = ValueHint::Other)]
    level_name: Option<String>,

    /// Directory template, appends server port [default: "Mammoth Server"]
    #[clap(short, long, value_hint = ValueHint::Other)]
    directory_template: Option<String>,

    /// Directories to sync to all instances [default: ["plugins"]]
    /// Any directory named "plugins" will sync to each server's plugins directory
    /// Other directories will sync to each server's root directory
    #[clap(long = "sync-dir", multiple_occurrences = true, multiple_values = false, value_hint = ValueHint::DirPath)]
    sync_dirs: Vec<PathBuf>,

    /// Graceful stop / restart timeout seconds [default: 10]
    #[clap(short, long, value_hint = ValueHint::Other)]
    timeout_secs: Option<u8>,

    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Clone, Parser)]
enum Command {
    #[clap(about = "Initialise and configure each server")]
    Init {
        /// World seed for all servers
        #[clap(short, long = "seed", value_hint = ValueHint::Other)]
        level_seed: Option<String>,

        /// Server Operators
        #[clap(
            short = 'o',
            long,
            multiple_occurrences = true,
            multiple_values = false,
            value_hint = ValueHint::Other
        )]
        ops: Vec<String>,

        /// Server Operators
        #[clap(
            short = 'w',
            long,
            multiple_occurrences = true,
            multiple_values = false, value_hint = ValueHint::Other
        )]
        white_list: Vec<String>,

        /// Additional server properties
        #[clap(
            short = 'p',
            long,
            multiple_occurrences = true,
            multiple_values = false,
            value_hint = ValueHint::Other
        )]
        server_properties: Vec<ServerProperty>,
    },

    #[clap(about = "Sync specified directories to all servers")]
    Sync {
        /// Clears plugins directory before syncing [default: false]
        #[clap(short = 'c', long)]
        clear_plugins: bool,
    },

    #[clap(about = "Update server .jar to the latest build for a given version")]
    UpdateServer,

    #[clap(about = "Resets each server's world")]
    ResetWorld,

    #[clap(about = "Remove all server directories")]
    Remove,

    #[clap(about = "Start all servers in the background")]
    Start {
        /// Maximum amount of RAM to allocate to each server [default: "1G"]
        #[clap(short = 'M', long, value_hint = ValueHint::Other)]
        max_memory: Option<ServerMemory>,

        /// Use Aikar's JVM flags [default: false]
        #[clap(long)]
        use_aikar_flags: Option<bool>,

        /// Additional JVM args. Overides Aikar's flags if set
        #[clap(long, value_hint = ValueHint::Other)]
        jvm_args: Option<String>,
    },

    #[clap(about = "Stop each background server process")]
    Stop,

    #[clap(about = "Restart all servers")]
    Restart {
        /// Maximum amount of RAM to allocate to each server [default: "1G"]
        #[clap(short = 'M', long, value_hint = ValueHint::Other)]
        max_memory: Option<ServerMemory>,

        /// Use Aikar's JVM flags [default: false]
        #[clap(long)]
        use_aikar_flags: Option<bool>,

        /// Additional JVM args. Overides Aikar's flags if set
        #[clap(long, value_hint = ValueHint::Other)]
        jvm_args: Option<String>,
    },

    #[clap(about = "Merge all world region files into a single folder")]
    Combine {
        #[clap(short = 'd', long)]
        world_diameter: Option<u32>,

        #[clap(short = 'W', long)]
        slice_width: Option<u32>,

        #[clap(long)]
        avoid_slicing_origin: Option<bool>,

        #[clap(short = 'r', long)]
        origin_radius: Option<u32>,

        #[clap(short = 'C', long)]
        combined_directory: Option<PathBuf>,
    },

    #[clap(about = "Remove irrelevant world files from each server")]
    Prune {
        #[clap(short = 'd', long)]
        world_diameter: Option<u32>,

        #[clap(short = 'W', long)]
        slice_width: Option<u32>,

        #[clap(long)]
        avoid_slicing_origin: Option<bool>,

        #[clap(short = 'r', long)]
        origin_radius: Option<u32>,

        #[clap(short = 'C', long)]
        combined_directory: Option<PathBuf>,
    },

    #[clap(about = "Generate shell completions")]
    Completions {
        /// CLI shell type
        #[clap(arg_enum)]
        shell: Shell,
    },
}

fn main() -> Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt()
        .with_target(false)
        .with_env_filter(format!("{}=trace", env!("CARGO_PKG_NAME")))
        .init();

    let config = config::read_config()?;
    let args = Args::parse();
    let global_args = config::global_args(config.global.unwrap_or_default(), args.clone());

    if global_args.server_count == 0 {
        warn!("no action taken as --server-count was set to 0");
        return Ok(());
    }

    if global_args.jar_version.is_empty() {
        error!("you must specify a server .jar version");
        std::process::exit(1);
    }

    match args.command {
        Command::Init {
            level_seed,
            ops,
            white_list,
            server_properties,
        } => {
            let init_args = config::init_args(
                config.init.unwrap_or_default(),
                level_seed,
                ops,
                white_list,
                server_properties,
            );

            if let Err(error) = init_args {
                error!("{}", error);
                std::process::exit(1);
            }

            let init_args = init_args.unwrap();
            if init_args.level_seed.is_empty() {
                error!("you must specify a level seed, else all servers will have different seeds");
                std::process::exit(1);
            }

            cmd_init::init(global_args, init_args)?
        }

        Command::Sync { clear_plugins } => cmd_sync::sync(global_args, clear_plugins)?,

        Command::UpdateServer => cmd_update_server::update_server(global_args)?,

        Command::ResetWorld => cmd_reset_world::reset_world(global_args)?,

        Command::Remove => cmd_remove::remove(global_args)?,

        Command::Start {
            max_memory,
            use_aikar_flags,
            jvm_args,
        } => {
            let start_args = config::start_args(
                config.start.unwrap_or_default(),
                max_memory,
                use_aikar_flags,
                jvm_args,
            );

            cmd_start_stop::start(global_args, start_args)?
        }

        Command::Stop => cmd_start_stop::stop(global_args)?,

        Command::Restart {
            max_memory,
            use_aikar_flags,
            jvm_args,
        } => {
            let start_args = config::start_args(
                config.start.unwrap_or_default(),
                max_memory,
                use_aikar_flags,
                jvm_args,
            );

            cmd_start_stop::restart(global_args, start_args)?
        }

        Command::Combine {
            world_diameter,
            slice_width,
            avoid_slicing_origin,
            origin_radius,
            combined_directory,
        } => {
            let world_management_args = config::world_management_args(
                config.world_management.unwrap_or_default(),
                world_diameter,
                slice_width,
                avoid_slicing_origin,
                origin_radius,
                combined_directory,
            );

            cmd_combine_prune::combine(global_args, world_management_args)?
        }

        Command::Prune {
            world_diameter,
            slice_width,
            avoid_slicing_origin,
            origin_radius,
            combined_directory,
        } => {
            let world_management_args = config::world_management_args(
                config.world_management.unwrap_or_default(),
                world_diameter,
                slice_width,
                avoid_slicing_origin,
                origin_radius,
                combined_directory,
            );

            cmd_combine_prune::prune(global_args, world_management_args)?
        }

        Command::Completions { shell } => {
            let mut app = Args::into_app();
            let app_name = app.get_name().to_owned();

            clap_complete::generate(shell, &mut app, app_name, &mut std::io::stdout());
        }
    }

    Ok(())
}
