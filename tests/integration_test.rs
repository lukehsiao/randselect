extern crate randselect;

use std::path::PathBuf;

#[test]
fn runs_without_error() {
    let mut test_args = randselect::Args {
        out_dir: PathBuf::from(r"./test/"),
        in_dir: PathBuf::from(r"."),
        num_files: 10,
        move_files: false,
        go: false,
        seed: Some(8),
    };
    if let Err(e) = randselect::run(&mut test_args) {
        panic!("Failed to run successfully: {:?}.", e)
    }
}

#[test]
fn runs_with_error() {
    let mut test_args = randselect::Args {
        out_dir: PathBuf::from(r"."),
        in_dir: PathBuf::from(r"."),
        num_files: 10,
        move_files: false,
        go: false,
        seed: Some(8),
    };
    if let Ok(_) = randselect::run(&mut test_args) {
        panic!("Should have failed.")
    }
}
