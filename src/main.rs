use std::process;

use clap::{crate_authors, crate_version, App, AppSettings, Arg};
use human_panic::setup_panic;
use log::warn;

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
        )
        .arg(
            Arg::with_name("out_dir")
                .short("o")
                .value_name("OUT_DIR")
                .help("The directory to output to. Will be created if it doesn't exist.")
                .required(true),
        )
        .arg(
            Arg::with_name("verbosity")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .arg(
            Arg::with_name("num_files")
                .short("n")
                .value_name("N")
                .help("The number of files to select.")
                .required(true),
        )
        .arg(
            Arg::with_name("seed")
                .short("s")
                .value_name("SEED")
                .help("The seed to use for the PRNG (u64).")
                .required(false),
        )
        .arg(
            Arg::with_name("move")
                .short("m")
                .help("Whether to move the selected files rather than copy.")
                .required(false),
        )
        .arg(
            Arg::with_name("go")
                .short("g")
                .long("go")
                .required(false)
                .help("Execute the copy or move. Specify a seed for deterministic behavior."),
        )
        .arg(
            Arg::with_name("no_color")
                .short("c")
                .long("no_color")
                .help("Disable colorized output. Only supported in Unix-like OSes."),
        )
        .get_matches();

    let num_files = match matches.value_of("num_files").unwrap().parse() {
        Ok(n) => n,
        Err(e) => {
            warn! {"{:?}\nDefaulting to 1 file.", e};
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

    let mut out_dir = String::from(
        matches
            .value_of("out_dir")
            .unwrap_or("No output directory."),
    );

    if out_dir.ends_with('/') {
        out_dir.pop();
    }

    // Return Config
    randselect::Args {
        out_dir,
        in_dir: String::from(matches.value_of("in_dir").unwrap_or("No input directory.")),
        num_files,
        seed,
        move_files: matches.is_present("move"),
        go: matches.is_present("go"),
        no_color: matches.is_present("no_color"),
        verbosity: match matches.occurrences_of("verbosity") {
            0 => 0,
            1 => 1,
            2 => 2,
            3 | _ => 3,
        },
    }
}

fn main() {
    setup_panic!();
    pretty_env_logger::init();
    let mut args = parse_args();

    if let Err(e) = randselect::run(&mut args) {
        warn!("{:?}: {:?}", e, args);
        process::exit(1);
    };
}
