use std::{
    env,
    path::{Path, PathBuf},
};

fn main() {
    // Gets all the arguments and skips the first one(which is the relative path of the binary from where
    // the command is called)
    let arguments: Vec<String> = env::args().skip(1).collect();

    // Source and destination path
    let paths = (PathBuf::from(&arguments[0]), PathBuf::from(&arguments[1]));
}
