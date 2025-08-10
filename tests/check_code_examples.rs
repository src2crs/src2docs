use src2docs::SourceCode;
use std::path::PathBuf;

fn examples_dir() -> PathBuf {
    PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("doc")
        .join("examples")
}

fn examples_dir_go() -> PathBuf {
    examples_dir().join("go")
}

#[test]
fn examples_dir_go_exists() {
    let examples_dir_go = examples_dir_go();
    assert!(
        examples_dir_go.exists(),
        "Examples directory does not exist: {}",
        examples_dir_go.display()
    );
    assert!(
        examples_dir_go.is_dir(),
        "{} is not a directory",
        examples_dir_go.display()
    );
}

#[test]
fn go_hello_world() {
    let code = SourceCode::go_hello_world();
    let examples_dir_go = examples_dir_go();
    let expected_path = examples_dir_go.join("hello_world").join("hello.go");
    let expected_code = std::fs::read_to_string(expected_path).expect("Could not read source file");

    assert_eq!(code.content, expected_code);
}
#[test]
fn go_task_fib() {
    let code = SourceCode::go_task_fib();
    let examples_dir_go = examples_dir_go();
    let expected_path = examples_dir_go.join("task_fib").join("fib.go");
    let expected_code = std::fs::read_to_string(expected_path).expect("Could not read source file");

    assert_eq!(code.content, expected_code);
}
#[test]
fn go_task_fib_test() {
    let code = SourceCode::go_task_fib_test();
    let examples_dir_go = examples_dir_go();
    let expected_path = examples_dir_go.join("task_fib").join("fib_test.go");
    let expected_code = std::fs::read_to_string(expected_path).expect("Could not read source file");

    assert_eq!(code.content, expected_code);
}
