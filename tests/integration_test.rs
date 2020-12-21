extern crate randselect;

#[test]
fn runs_without_error() {
    let mut test_args = randselect::Args {
        out_dir: "./test/".to_string(),
        in_dir: ".".to_string(),
        num_files: 10,
        move_files: false,
        go: false,
        no_color: false,
        seed: Some(8),
    };
    if let Err(e) = randselect::run(&mut test_args) {
        panic!("Failed to run successfully: {:?}.", e)
    }
}

#[test]
fn runs_with_error() {
    let mut test_args = randselect::Args {
        out_dir: ".".to_string(),
        in_dir: ".".to_string(),
        num_files: 10,
        move_files: false,
        go: false,
        no_color: false,
        seed: Some(8),
    };
    if let Ok(_) = randselect::run(&mut test_args) {
        panic!("Should have failed.")
    }
}
