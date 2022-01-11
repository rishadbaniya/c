use std::{
    env,
    path::{Path, PathBuf},
};

use std::fs;

fn main() {
    // Gets all the arguments and skips the first one(which is the relative path of the binary from where
    // the command is called)
    let arguments: Vec<String> = env::args().skip(1).collect();

    // Source and destination path
    let paths = (PathBuf::from(&arguments[0]), PathBuf::from(&arguments[1]));

    // Copies a regular file from source to destination
    fs::copy(paths.0, paths.1);
}
