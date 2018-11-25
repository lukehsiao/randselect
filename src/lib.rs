//! # randselect
//! This crate provides a simple commandline utility for randomly selecting N
//! files from a directory and copying/moving them to a target directory.
//!
//! `randselect` operates (inefficiently) by generating a random permutation of
//! the files in a given directory, then moving/copying the first N files in
//! the resulting permutation to a target directory.
//!
//! ```txt
//! randselect
//! Tool for randomly selecting files from a directory.
//!
//! USAGE:
//!     randselect [FLAGS] [OPTIONS] -i <IN_DIR> -n <N> -o <OUT_DIR>
//!
//! FLAGS:
//!     -h, --help        Prints help information
//!     -m                Whether to move the selected files rather than copy.
//!     -c, --no_color    Disable colorized output. Only supported in Unix-like OSes.
//!     -V, --version     Prints version information
//!     -v                Sets the level of verbosity
//!
//! OPTIONS:
//!     -i <IN_DIR>         The input directory to select from.
//!     -n <N>              The number of files to select.
//!     -o <OUT_DIR>        The directory to output to. Will be created if it doesn't exist.
//!     -s <SEED>           The seed to use for the PRNG (u64).
//! ```

extern crate chrono;
#[macro_use]
extern crate log;
extern crate fern;
extern crate rand;

pub mod utils;

use rand::prelude::{SeedableRng, SliceRandom, StdRng};
use rand::FromEntropy;
use std::fs;
use std::io::{Error, ErrorKind};

#[derive(Debug)]
pub struct Args {
    pub verbosity: u8,
    pub out_dir: String,
    pub in_dir: String,
    pub num_files: usize,
    pub move_files: bool,
    pub dry_run: bool,
    pub no_color: bool,
    pub seed: Option<u64>,
}

/// Return a shuffled vector of paths based on the seed, if provided.
fn get_shuffled_paths(args: &Args) -> Result<Vec<fs::DirEntry>, Error> {
    match fs::read_dir(&args.in_dir) {
        Ok(paths) => {
            // Use seed if provided, else random entropy
            let mut rng: StdRng = match args.seed {
                // NOTE: Not cryptographically secure, but good enough for us.
                Some(seed) => SeedableRng::seed_from_u64(seed),
                None => StdRng::from_entropy(),
            };

            let mut vec_paths: Vec<_> = paths.map(|r| r.unwrap()).collect();

            // Generate a random permutation of the files
            vec_paths.shuffle(&mut rng);
            trace!{"Shuffled: {:#?}", vec_paths};
            return Ok(vec_paths);
        }
        Err(e) => Err(e),
    }
}

fn paths_are_valid(in_dir: &str, out_dir: &str) -> Result<(), Error> {
    let in_path = fs::canonicalize(in_dir)?;
    if !in_path.is_dir() {
        error!{"Input directory is not a directory: {}", in_dir};
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "Input directory is not a directory.",
        ));
    }

    // Only check output directory if it exists
    if let Ok(out_path) = fs::canonicalize(out_dir) {
        if in_path == out_path {
            error!(
                "The output directory cannot be the same as the input directory.\n{} == {}",
                in_path.to_str().unwrap(),
                out_path.to_str().unwrap()
            );
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Output and input directory were the same.",
            ));
        }
    }

    Ok(())
}

/// The primary driver of the library to process the provided Args.
pub fn run(args: &Args) -> Result<(), Error> {
    debug!{"Input args: {:#?}", args};

    paths_are_valid(args.in_dir.as_str(), args.out_dir.as_str())?;

    match get_shuffled_paths(args) {
        Ok(paths) => {
            let selected_files: Vec<&fs::DirEntry> = paths.iter().take(args.num_files).collect();
            if args.dry_run {
                // If debug, just print which files would be selected
                warn!{"This is a DRY RUN."};
                for file in selected_files {
                    print!{"{}\n", file.path().display()};
                }
            } else {
                debug!{"{:#?}", selected_files};
                fs::create_dir_all(&args.out_dir).unwrap();
                for file in selected_files {
                    // Copy file
                    let dest = format!(
                        "{}/{}",
                        args.out_dir,
                        file.file_name().into_string().unwrap()
                    );
                    fs::copy(file.path(), dest).unwrap();

                    // Delete file if move
                    if args.move_files {
                        fs::remove_file(file.path()).unwrap();
                    }
                }
            }
            return Ok(());
        }
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_valid_paths() {
        paths_are_valid(".", "/tmp/randselect").expect("Paths are valid.");
    }

    #[test]
    fn test_invalid_paths() {
        if let Ok(_) = paths_are_valid(".", ".") {
            panic!("Should have failed with same paths");
        }
    }

}
