# randselect
[![Crates.io](https://img.shields.io/crates/v/randselect.svg)](https://crates.io/crates/randselect)
[![docs.rs](https://docs.rs/randselect/badge.svg)](https://docs.rs/randselect)
[![Travis (.com)](https://img.shields.io/travis/com/lukehsiao/randselect.svg)](https://travis-ci.com/lukehsiao/randselect)

This crate provides a simple command line utility for randomly selecting N
files from a directory and copying or moving them to a target directory.

`randselect` operates (inefficiently) by generating a random permutation of
the files in a given directory, then moving or copying the first N files in
the resulting permutation to a target directory.

Could you do this with a few lines of bash? Almost certainly! This was just an
excuse to write some Rust.

## Install

You can install this with Cargo:

```
$ cargo install randselect
```

## Usage
```
randselect
Tool for randomly selecting files from a directory.

USAGE:
    randselect [FLAGS] [OPTIONS] -i <IN_DIR> -n <N> -o <OUT_DIR>

FLAGS:
    -g, --go          Execute the copy or move. Specify a seed for deterministic behavior.
    -h, --help        Prints help information
    -m                Whether to move the selected files rather than copy.
    -c, --no_color    Disable colorized output. Only supported in Unix-like OSes.
    -V, --version     Prints version information

OPTIONS:
    -i <IN_DIR>         The input directory to select from.
    -n <N>              The number of files to select.
    -o <OUT_DIR>        The directory to output to. Will be created if it doesn't exist.
    -s <SEED>           The seed to use for the PRNG (u64).
```
