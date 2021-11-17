use clap::Parser;
use color_eyre::Result;
use tracing::{error, warn};
use utils::ServerProperty;

mod cmd_init;
mod cmd_remove;
mod cmd_start_stop;
mod cmd_sync_plugins;
mod config;
mod paper;
mod utils;

#[derive(Debug, Clone, Parser)]
#[clap(about, version)]
pub struct Args {
    /// Number of servers to initialise [default: 2]
    #[clap(short = 'c', long)]
    server_count: Option<u8>,

    /// Server port to start counting at [default: 25565]
    #[clap(short = 'p', long)]
    start_port: Option<u16>,

    /// Directory template, appends server port [default: "Mammoth Server"]
    #[clap(short, long)]
    directory_template: Option<String>,

    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Clone, Parser)]
enum Command {
    #[clap(about = "Initialise and configure test servers")]
    Init {
        /// Paper version [default: "1.17.1"]
        #[clap(short = 'P', long)]
        paper_version: Option<String>,

        /// World seed for all servers
        #[clap(short, long = "seed")]
        level_seed: Option<String>,

        /// Server Operators
        #[clap(
            short = 'o',
            long,
            multiple_occurrences = true,
            multiple_values = false
        )]
        ops: Vec<String>,

        /// Server Operators
        #[clap(
            short = 'w',
            long,
            multiple_occurrences = true,
            multiple_values = false
        )]
        white_list: Vec<String>,

        /// Don't copy Plugins directory
        #[clap(short, long)]
        skip_plugins: Option<bool>,

        /// Don't copy bukkit.yml
        #[clap(long)]
        no_copy_bukkit: Option<bool>,

        /// Don't copy spigot.yml
        #[clap(long)]
        no_copy_spigot: Option<bool>,

        /// Don't copy paper.yml
        #[clap(long)]
        no_copy_paper: Option<bool>,

        /// Additional server properties
        #[clap(
            short = 'p',
            long,
            multiple_occurrences = true,
            multiple_values = false
        )]
        server_properties: Vec<ServerProperty>,
    },

    #[clap(about = "Sync plugins to all test servers")]
    SyncPlugins,

    #[clap(about = "Remove test servers")]
    Remove,

    #[clap(about = "Start test servers")]
    Start {
        /// Maximum amount of RAM to allocate to each server [default: "1G"]
        #[clap(short = 'M', long)]
        max_memory: Option<String>,
    },

    #[clap(about = "Stop test servers")]
    Stop,

    #[clap(about = "Restart test servers")]
    Restart {
        /// Maximum amount of RAM to allocate to each server [default: "1G"]
        #[clap(short = 'M', long)]
        max_memory: Option<String>,
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

    match args.command {
        Command::Init {
            paper_version,
            level_seed,
            ops,
            white_list,
            skip_plugins,
            no_copy_bukkit,
            no_copy_spigot,
            no_copy_paper,
            server_properties,
        } => {
            let init_args = config::init_args(
                config.init.unwrap_or_default(),
                paper_version,
                level_seed,
                ops,
                white_list,
                skip_plugins,
                no_copy_bukkit,
                no_copy_spigot,
                no_copy_paper,
                server_properties,
            )?;

            if init_args.level_seed.is_empty() {
                error!("--seed must be set else all servers will have different seeds");
                std::process::exit(1);
            }

            cmd_init::init(global_args, init_args)?
        }

        Command::SyncPlugins => cmd_sync_plugins::sync_plugins(global_args)?,

        Command::Remove => cmd_remove::remove(global_args)?,

        Command::Start { max_memory } => {
            let start_args = config::start_args(config.start.unwrap_or_default(), max_memory);
            cmd_start_stop::start(global_args, start_args)?
        }

        Command::Stop => cmd_start_stop::stop(global_args)?,

        Command::Restart { max_memory } => {
            let start_args = config::start_args(config.start.unwrap_or_default(), max_memory);
            cmd_start_stop::restart(global_args, start_args)?
        }
    }

    Ok(())
}
