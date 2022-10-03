use anyhow::Result;
use clap::Parser;
use human_panic::setup_panic;

use randselect::Cli;

fn main() -> Result<()> {
    setup_panic!();
    pretty_env_logger::init();
    let cli = Cli::parse();

    randselect::run(&cli)?;

    Ok(())
}
