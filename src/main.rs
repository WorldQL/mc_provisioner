use clap::Parser;
use color_eyre::Result;

mod cmd_init;

#[derive(Debug, Parser)]
enum Args {
    // Initialise test servers
    Init {
        /// Server port to start counting at
        #[clap(short = 'p', long, default_value = "25565")]
        start_port: u16,

        /// World seed for all servers
        #[clap(short, long)]
        level_seed: String,

        /// MOTD Template, prepends server index
        #[clap(short, long, default_value = "Mammoth Server")]
        motd_template: String,

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
}

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_env_filter(format!("{}=trace", env!("CARGO_PKG_NAME")))
        .init();

    let args = Args::parse();
    match args {
        Args::Init {
            start_port,
            level_seed,
            motd_template,
            skip_plugins,
            no_copy_bukkit,
            no_copy_spigot,
            no_copy_paper,
        } => cmd_init::init(
            start_port,
            level_seed,
            motd_template,
            skip_plugins,
            no_copy_bukkit,
            no_copy_spigot,
            no_copy_paper,
        )?,
    }

    Ok(())
}
