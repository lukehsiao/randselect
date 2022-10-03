use std::fs;

use colored::Colorize;
use log::{debug, error, trace};
use rand::prelude::{SeedableRng, SliceRandom, StdRng};
use std::io;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RandSelectError {
    #[error(transparent)]
    IoError(#[from] io::Error),
}

#[derive(Debug, clap::Parser)]
#[command(version, about)]
pub struct Cli {
    /// The input directory to select from.
    #[arg(value_name("IN_DIR"))]
    pub in_dir: PathBuf,

    /// The directory to output to. Will be created if it doesn't exist.
    #[arg(value_name("OUT_DIR"))]
    pub out_dir: PathBuf,

    /// The number of files to select.
    #[arg(short, long, default_value = "1")]
    pub num_files: usize,

    /// Whether to move the files from IN_DIR to OUT_DIR, rather than cp.
    #[arg(short, long)]
    pub move_files: bool,

    /// Execute the copy or move. Specify a seed for deterministic behavior.
    #[arg(short, long)]
    pub go: bool,

    /// The seed to use for the PRNG (u64).
    #[arg(short, long)]
    pub seed: Option<u64>,
}

/// Return a shuffled vector of paths based on the seed, if provided.
fn get_shuffled_paths(cli: &Cli) -> Result<Vec<fs::DirEntry>, RandSelectError> {
    match fs::read_dir(&cli.in_dir) {
        Ok(paths) => {
            // Use seed if provided, else random entropy
            let mut rng: StdRng = match cli.seed {
                // NOTE: Not cryptographically secure, but good enough for us.
                Some(seed) => SeedableRng::seed_from_u64(seed),
                None => StdRng::from_entropy(),
            };

            // Only consider files, not directories
            let mut vec_paths: Vec<_> = paths
                .filter_map(|p| match p {
                    Ok(entry) if entry.file_type().ok()?.is_file() => Some(entry),
                    _ => None,
                })
                .collect();

            // Generate a random permutation of the files
            vec_paths.shuffle(&mut rng);
            trace! {"Shuffled: {:#?}", vec_paths};
            Ok(vec_paths)
        }
        Err(e) => Err(e.into()),
    }
}

fn paths_are_valid(in_dir: &Path, out_dir: &Path) -> Result<(), RandSelectError> {
    if !in_dir.is_dir() {
        error!("Input directory is not a directory: {}", in_dir.display());
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Input directory is not a directory.",
        )
        .into());
    }

    if in_dir == out_dir {
        error!(
            "The output directory cannot be the same as the input directory.\n{} == {}",
            in_dir.display(),
            out_dir.display()
        );
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Output and input directory were the same.",
        )
        .into());
    }

    Ok(())
}

/// The primary driver of the library to process the provided Cli.
pub fn run(cli: &Cli) -> Result<(), RandSelectError> {
    debug!("Input args: {:#?}", cli);

    paths_are_valid(cli.in_dir.as_path(), cli.out_dir.as_path())?;

    match get_shuffled_paths(cli) {
        Ok(paths) => {
            let selected_files: Vec<&fs::DirEntry> = paths.iter().take(cli.num_files).collect();
            for file in selected_files {
                let dest = format!(
                    "{}/{}",
                    cli.out_dir.display(),
                    file.file_name().into_string().unwrap()
                );
                // Delete file if move
                if cli.move_files {
                    println!("{} {}", "--".red(), file.path().to_str().unwrap().red());
                }
                println!("{} {}", "++".green(), dest.green());
                if cli.go {
                    // Create output dir
                    fs::create_dir_all(&cli.out_dir).unwrap();

                    // Copy file
                    fs::copy(file.path(), &dest).unwrap();

                    // Delete file if move
                    if cli.move_files {
                        fs::remove_file(file.path()).unwrap();
                    }
                }
            }
            if !cli.go {
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
        let rand_path =
            String::from_utf8(thread_rng().sample_iter(&Alphanumeric).take(18).collect()).unwrap();
        let rand_output = Path::new(&rand_path);

        paths_are_valid(Path::new("."), rand_output).expect("Paths are valid.");
    }

    #[test]
    fn test_invalid_paths() {
        if let Ok(_) = paths_are_valid(Path::new("."), Path::new(".")) {
            panic!("Should have failed with same paths");
        }
    }

    #[test]
    fn verify_app() {
        use clap::CommandFactory;
        Cli::command().debug_assert()
    }
}
