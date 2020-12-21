//! # randselect
//! This crate provides a simple command line utility for randomly selecting N
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
//!     -g, --go          Execute the copy or move. Specify a seed for deterministic behavior.
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

use std::fs;
use std::io::{Error, ErrorKind};

use colored::Colorize;
use log::{debug, error, trace};
use rand::prelude::{SeedableRng, SliceRandom, StdRng};

#[derive(Debug)]
pub struct Args {
    pub verbosity: u8,
    pub out_dir: String,
    pub in_dir: String,
    pub num_files: usize,
    pub move_files: bool,
    pub go: bool,
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
            trace! {"Shuffled: {:#?}", vec_paths};
            Ok(vec_paths)
        }
        Err(e) => Err(e),
    }
}

fn paths_are_valid(in_dir: &str, out_dir: &str) -> Result<(), Error> {
    let in_path = fs::canonicalize(in_dir)?;
    if !in_path.is_dir() {
        error! {"Input directory is not a directory: {}", in_dir};
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
pub fn run(args: &mut Args) -> Result<(), Error> {
    debug! {"Input args: {:#?}", args};

    paths_are_valid(args.in_dir.as_str(), args.out_dir.as_str())?;

    if args.no_color {
        colored::control::set_override(false);
    }

    match get_shuffled_paths(args) {
        Ok(paths) => {
            let selected_files: Vec<&fs::DirEntry> = paths.iter().take(args.num_files).collect();
            for file in selected_files {
                let dest = format!(
                    "{}/{}",
                    args.out_dir,
                    file.file_name().into_string().unwrap()
                );
                // Delete file if move
                if args.move_files {
                    println!("{} {}", "--".red(), file.path().to_str().unwrap().red());
                }
                println!("{} {}", "++".green(), dest.green());
                if args.go {
                    // Create output dir
                    fs::create_dir_all(&args.out_dir).unwrap();

                    // Copy file
                    fs::copy(file.path(), &dest).unwrap();

                    // Delete file if move
                    if args.move_files {
                        fs::remove_file(file.path()).unwrap();
                    }
                }
            }
            if !args.go {
                println!("Re-run randselect with --go to write these changes to the filesystem.");
            }
            Ok(())
        }
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};

    #[test]
    fn test_valid_paths() {
        let rand_output: String =
            String::from_utf8(thread_rng().sample_iter(&Alphanumeric).take(18).collect()).unwrap();

        paths_are_valid(".", &rand_output).expect("Paths are valid.");
    }

    #[test]
    fn test_invalid_paths() {
        if let Ok(_) = paths_are_valid(".", ".") {
            panic!("Should have failed with same paths");
        }
    }
}
