use std::process;

use human_panic::setup_panic;
use log::warn;
use structopt::StructOpt;

use randselect::Args;

fn main() {
    setup_panic!();
    pretty_env_logger::init();
    let mut args = Args::from_args();

    if let Err(e) = randselect::run(&mut args) {
        warn!("{:?}: {:?}", e, args);
        process::exit(1);
    };
}
