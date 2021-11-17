use clap::Parser;
use color_eyre::Result;
use tracing::warn;

mod cmd_init;
mod cmd_remove;
mod cmd_start_stop;
mod cmd_sync_plugins;
mod paper;
mod utils;

#[derive(Debug, Parser)]
#[clap(about, version)]
struct Args {
    /// Number of servers to initialise
    #[clap(short = 'c', long, default_value = "2")]
    server_count: u8,

    /// Server port to start counting at
    #[clap(short = 'p', long, default_value = "25565")]
    start_port: u16,

    /// Directory template, appends server port
    #[clap(short, long, default_value = "Mammoth Server")]
    directory_template: String,

    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Parser)]
enum Command {
    #[clap(about = "Initialise and configure test servers")]
    Init {
        /// Paper version
        #[clap(short, long, default_value = "1.17.1")]
        paper_version: String,

        /// World seed for all servers
        #[clap(short, long = "seed")]
        level_seed: String,

        /// Don't copy Plugins directory
        #[clap(short, long)]
        skip_plugins: bool,

        /// Don't copy bukkit.yml
        #[clap(long)]
        no_copy_bukkit: bool,

        /// Don't copy spigot.yml
        #[clap(long)]
        no_copy_spigot: bool,

        /// Don't copy paper.yml
        #[clap(long)]
        no_copy_paper: bool,
    },

    #[clap(about = "Sync plugins to all test servers")]
    SyncPlugins,

    #[clap(about = "Remove test servers")]
    Remove,

    #[clap(about = "Start test servers")]
    Start {
        /// Maximum amount of RAM to allocate to each server
        #[clap(short = 'M', long, default_value = "1G")]
        max_memory: String,
    },

    #[clap(about = "Stop test servers")]
    Stop,

    #[clap(about = "Restart test servers")]
    Restart {
        /// Maximum amount of RAM to allocate to each server
        #[clap(short = 'M', long, default_value = "1G")]
        max_memory: String,
    },
}

fn main() -> Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt()
        .with_target(false)
        .with_env_filter(format!("{}=trace", env!("CARGO_PKG_NAME")))
        .init();

    let args = Args::parse();
    if args.server_count == 0 {
        warn!("no action taken as --server-count was set to 0");
        return Ok(());
    }

    match args.command {
        Command::Init {
            paper_version,
            level_seed,
            skip_plugins,
            no_copy_bukkit,
            no_copy_spigot,
            no_copy_paper,
        } => cmd_init::init(
            args.server_count,
            args.start_port,
            args.directory_template,
            paper_version,
            level_seed,
            skip_plugins,
            (no_copy_bukkit, no_copy_spigot, no_copy_paper),
        )?,

        Command::SyncPlugins => cmd_sync_plugins::sync_plugins(
            args.server_count,
            args.start_port,
            args.directory_template,
        )?,

        Command::Remove => {
            cmd_remove::remove(args.server_count, args.start_port, args.directory_template)?
        }

        Command::Start { max_memory } => cmd_start_stop::start(
            args.server_count,
            args.start_port,
            args.directory_template,
            max_memory,
        )?,

        Command::Stop => {
            cmd_start_stop::stop(args.server_count, args.start_port, args.directory_template)?
        }

        Command::Restart { max_memory } => cmd_start_stop::restart(
            args.server_count,
            args.start_port,
            args.directory_template,
            max_memory,
        )?,
    }

    Ok(())
}
