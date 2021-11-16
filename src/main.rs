use clap::Parser;
use color_eyre::Result;

#[derive(Debug, Parser)]
struct Args {
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
}

fn main() -> Result<()> {
    let args = Args::parse();
    dbg!(&args);

    Ok(())
}
