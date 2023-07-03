# randselect
[![Crates.io](https://img.shields.io/crates/v/randselect)](https://crates.io/crates/randselect)
[![GitHub Workflow Status (with event)](https://img.shields.io/github/actions/workflow/status/lukehsiao/randselect/rust.yml)](https://github.com/lukehsiao/randselect/actions/workflows/rust.yml)
[![Crates.io](https://img.shields.io/crates/l/randselect)](https://github.com/lukehsiao/randselect/blob/main/LICENSE)

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
    randselect [OPTIONS] <IN_DIR> <OUT_DIR>

ARGS:
    <IN_DIR>     The input directory to select from
    <OUT_DIR>    The directory to output to. Will be created if it doesn't exist

OPTIONS:
    -g, --go                       Execute the copy or move. Specify a seed for deterministic
                                   behavior
    -h, --help                     Print help information
    -m, --move-files               Whether to move the files from IN_DIR to OUT_DIR, rather than cp
    -n, --num-files <NUM_FILES>    The number of files to select [default: 1]
    -s, --seed <SEED>              The seed to use for the PRNG (u64)
    -V, --version                  Print version information
```
