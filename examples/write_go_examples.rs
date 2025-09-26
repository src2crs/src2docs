use std::{env, path::PathBuf};

use src2docs::{GoExample, SourceCode};

fn main() {
    write_example(GoExample::DemoHello);
    write_example(GoExample::TaskFib);
    write_example(GoExample::TaskFibTest);
}

fn write_example(example: GoExample) {
    create_out_dir();
    let out_dir = out_dir_path();
    let file_name = format!("{:?}.go", example);

    let source_code: SourceCode = example.to_source_code();
    std::fs::write(out_dir.join(file_name), source_code.to_string()).unwrap();
}

fn out_dir_path() -> PathBuf {
    let package_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let package_dir = std::path::PathBuf::from(&package_dir);
    let examples_dir = package_dir.join("examples");
    examples_dir.join("out")
}

/// Create the output directory if it doesn't exist.
/// Also creates a .gitignore file to ignore all files in the directory.
fn create_out_dir() {
    let out_dir = out_dir_path();
    if !out_dir.exists() {
        std::fs::create_dir_all(&out_dir).unwrap();
        std::fs::write(out_dir.join(".gitignore"), "*\n").unwrap();
    }
}
