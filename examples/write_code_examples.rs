use std::{env, path::PathBuf};

use src2docs::examples::{Example, GoExample};

fn main() {
    create_out_dirs();

    GoExample::write_all(out_dir_go_path()).unwrap();
}

/// The output directory for the generated files.
fn out_dir_path() -> PathBuf {
    let package_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let package_dir = std::path::PathBuf::from(&package_dir);
    let examples_dir = package_dir.join("examples");
    examples_dir.join("out")
}

/// The path to the subdirectory for Go examples.
fn out_dir_go_path() -> PathBuf {
    out_dir_path().join("go")
}

/// Create the output directories if they don't exist.
/// Also creates a .gitignore file to ignore all files in the main directory.
fn create_out_dirs() {
    let out_dir = out_dir_path();
    if !out_dir.exists() {
        std::fs::create_dir_all(&out_dir).unwrap();
        println!("Created output directory: {}", out_dir.display());
    }
    let ignore_file = out_dir.join(".gitignore");
    if !ignore_file.exists() {
        std::fs::write(ignore_file, "*\n").unwrap();
        println!("Created .gitignore file in output directory");
    }
    let out_dir_go = out_dir_go_path();
    if !out_dir_go.exists() {
        std::fs::create_dir_all(&out_dir_go).unwrap();
        println!("Created Go output directory: {}", out_dir_go.display());
    }
}
