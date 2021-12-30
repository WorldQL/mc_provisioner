use clap::{IntoApp, Parser, ValueHint};
use clap_generate::Shell;
use color_eyre::Result;
use tracing::{error, warn};
use utils::{ServerMemory, ServerProperty};

mod cmd_init;
mod cmd_remove;
mod cmd_reset_world;
mod cmd_start_stop;
mod cmd_sync_plugins;
mod config;
mod paper;
mod utils;

#[derive(Debug, Clone, Parser)]
#[clap(about, version)]
pub struct Args {
    /// Number of servers to initialise [default: 2]
    #[clap(short = 'c', long, value_hint = ValueHint::Other)]
    server_count: Option<u8>,

    /// Server port to start counting at [default: 25565]
    #[clap(short = 'p', long, value_hint = ValueHint::Other)]
    start_port: Option<u16>,

    /// Directory template, appends server port [default: "Mammoth Server"]
    #[clap(short, long, value_hint = ValueHint::Other)]
    directory_template: Option<String>,

    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Clone, Parser)]
enum Command {
    #[clap(about = "Initialise and configure each server")]
    Init {
        /// Paper version [default: "1.17.1"]
        #[clap(short = 'P', long, value_hint = ValueHint::Other)]
        paper_version: Option<String>,

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

        /// Don't copy Plugins directory
        #[clap(short, long, value_hint = ValueHint::Other)]
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
            multiple_values = false,
            value_hint = ValueHint::Other
        )]
        server_properties: Vec<ServerProperty>,
    },

    #[clap(about = "Sync plugins directory to all servers")]
    SyncPlugins,

    #[clap(about = "Resets each server's world")]
    ResetWorld,

    #[clap(about = "Remove all server directories")]
    Remove,

    #[clap(about = "Start all servers in the background")]
    Start {
        /// Maximum amount of RAM to allocate to each server [default: "1G"]
        #[clap(short = 'M', long)]
        max_memory: Option<ServerMemory>,
    },

    #[clap(about = "Stop each background server process")]
    Stop,

    #[clap(about = "Restart all servers")]
    Restart {
        /// Maximum amount of RAM to allocate to each server [default: "1G"]
        #[clap(short = 'M', long, value_hint = ValueHint::Other)]
        max_memory: Option<ServerMemory>,
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
            );

            if let Err(error) = init_args {
                error!("{}", error);
                std::process::exit(1);
            }

            let init_args = init_args.unwrap();
            if init_args.level_seed.is_empty() {
                error!("--seed must be set else all servers will have different seeds");
                std::process::exit(1);
            }

            cmd_init::init(global_args, init_args)?
        }

        Command::SyncPlugins => cmd_sync_plugins::sync_plugins(global_args)?,

        Command::ResetWorld => cmd_reset_world::reset_world(global_args)?,

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

        Command::Completions { shell } => {
            let mut app = Args::into_app();
            let app_name = app.get_name().to_string();

            clap_generate::generate(shell, &mut app, app_name, &mut std::io::stdout());
        }
    }

    Ok(())
}
