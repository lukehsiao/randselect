# randselect
[![Crates.io](https://img.shields.io/crates/v/randselect.svg)](https://crates.io/crates/randselect)
[![docs.rs](https://docs.rs/randselect/badge.svg)](https://docs.rs/randselect)
[![GitHub Workflow Status](https://img.shields.io/github/workflow/status/lukehsiao/randselect/rust)](https://github.com/lukehsiao/randselect/actions)

This crate provides a simple command line utility for randomly selecting N files
from a directory and copying or moving them to a target directory.

`randselect` operates (inefficiently) by generating a random permutation of the
files in a given directory, then moving or copying the first N files in the
resulting permutation to a target directory.

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
    randselect [FLAGS] [OPTIONS] <OUT_DIR> <IN_DIR>

FLAGS:
    -g, --go            Execute the copy or move. Specify a seed for deterministic behavior
    -h, --help          Prints help information
    -m, --move-files    Whether to move the files from IN_DIR to OUT_DIR, rather than cp
    -V, --version       Prints version information

OPTIONS:
    -n, --num-files <num-files>    The number of files to select [default: 1]
    -s, --seed <seed>              The seed to use for the PRNG (u64)

ARGS:
    <OUT_DIR>    The directory to output to. Will be created if it doesn't exist
    <IN_DIR>     The input directory to select from
```
