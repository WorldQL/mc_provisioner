use clap::Parser;
use color_eyre::Result;

mod cmd_init;

#[derive(Debug, Parser)]
struct Args {
    /// Number of servers to initialise
    #[clap(short = 'c', long, default_value = "2")]
    server_count: u8,

    /// Server port to start counting at
    #[clap(short = 'p', long, default_value = "25565")]
    start_port: u16,

    /// MOTD Template, prepends server index
    #[clap(short, long, default_value = "Mammoth Server")]
    motd_template: String,

    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Parser)]
enum Command {
    #[clap(about = "Initialise and configure test servers")]
    Init {
        /// World seed for all servers
        #[clap(short, long)]
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

    #[clap(about = "Remove test servers")]
    Remove,

    #[clap(about = "Start test servers")]
    Start,

    #[clap(about = "Stop test servers")]
    Stop,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt()
        .with_target(false)
        .with_env_filter(format!("{}=trace", env!("CARGO_PKG_NAME")))
        .init();

    let args = Args::parse();
    match args.command {
        Command::Init {
            level_seed,
            skip_plugins,
            no_copy_bukkit,
            no_copy_spigot,
            no_copy_paper,
        } => cmd_init::init(
            args.server_count,
            args.start_port,
            args.motd_template,
            level_seed,
            skip_plugins,
            (no_copy_bukkit, no_copy_spigot, no_copy_paper),
        )?,

        Command::Remove => {
            todo!()
        }

        Command::Start => {
            todo!()
        }

        Command::Stop => {
            todo!()
        }
    }

    Ok(())
}
