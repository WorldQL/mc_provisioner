use clap::Parser;
use color_eyre::Result;

#[derive(Debug, Parser)]
struct Args {}

fn main() -> Result<()> {
    let args = Args::parse();
    dbg!(&args);

    Ok(())
}
