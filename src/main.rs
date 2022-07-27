use std::process;

use clap::Parser;
use human_panic::setup_panic;
use log::warn;

use randselect::Cli;

fn main() {
    setup_panic!();
    pretty_env_logger::init();
    let cli = Cli::parse();

    if let Err(e) = randselect::run(&cli) {
        warn!("{:?}: {:?}", e, cli);
        process::exit(1);
    };
}
