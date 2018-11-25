extern crate chrono;
#[macro_use]
extern crate clap;
extern crate fern;
#[macro_use]
extern crate log;
extern crate rand;

mod utils;

use std::env;
use std::process;

use clap::{App, AppSettings, Arg};

use utils::setup_logging;

/// Get the URL provided on the commandline.
fn parse_args() -> randselect::Args {
    let matches = App::new("randselect")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Tool for randomly selecting files from a directory.")
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::ColoredHelp)
        .arg(
            Arg::with_name("in_dir")
                .short("i")
                .value_name("IN_DIR")
                .help("The input directory to select from.")
                .required(true),
        ).arg(
            Arg::with_name("out_dir")
                .short("o")
                .value_name("OUT_DIR")
                .help("The directory to output to. Will be created if it doesn't exist.")
                .required(true),
        ).arg(
            Arg::with_name("verbosity")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
        ).arg(
            Arg::with_name("num_files")
                .short("n")
                .value_name("N")
                .help("The number of files to select.")
                .required(true),
        ).arg(
            Arg::with_name("seed")
                .short("s")
                .value_name("SEED")
                .help("The seed to use for the PRNG (u64).")
                .required(false),
        ).arg(
            Arg::with_name("move")
                .short("m")
                .help("Whether to move the selected files rather than copy.")
                .required(false),
        ).arg(
            Arg::with_name("dry_run")
                .short("d")
                .help("Don't copy or move, just print what would be moved.")
                .required(false),
        ).arg(
            Arg::with_name("no_color")
                .short("c")
                .long("no_color")
                .help("Disable colorized output. Only supported in Unix-like OSes."),
        ).get_matches();

    // Per no-color.org standards, if the NO_COLOR environment variable is set,
    // set to no color regardless of the flags.
    let no_color = match env::var("NO_COLOR") {
        Ok(_) => true,
        Err(_) => matches.is_present("no_color"),
    };

    let num_files = match matches.value_of("num_files").unwrap().parse() {
        Ok(n) => n,
        Err(e) => {
            warn!{"{:?}\nDefaulting to 1 file.", e};
            1
        }
    };

    let seed = match matches.value_of("seed") {
        Some(seed_str) => match seed_str.parse() {
            Ok(n) => Some(n),
            Err(_) => None,
        },
        None => None,
    };

    // Return Config
    randselect::Args {
        out_dir: String::from(
            matches
                .value_of("out_dir")
                .unwrap_or("No output directory."),
        ),
        in_dir: String::from(matches.value_of("in_dir").unwrap_or("No input directory.")),
        num_files: num_files,
        seed: seed,
        move_files: matches.is_present("move"),
        dry_run: matches.is_present("dry_run"),
        no_color: no_color,
        verbosity: match matches.occurrences_of("verbosity") {
            0 => 0,
            1 => 1,
            2 => 2,
            3 | _ => 3,
        },
    }
}

fn main() {
    let args = parse_args();
    setup_logging(args.verbosity, args.no_color).expect("Unable to setup logging.");
    if let Err(e) = randselect::run(&args) {
        warn!("{:?}: {:?}", e, args);
        process::exit(1);
    };
}
