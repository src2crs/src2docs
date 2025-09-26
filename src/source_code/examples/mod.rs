use super::SourceCode;

use crate::macros::crate_navigation::example_file_content;

/// Example source code snippets for testing and documentation purposes.
impl SourceCode {
    /// A "Hello World" program in Go.
    pub fn example_go_hello_world() -> Self {
        let code = example_file_content!("go", "hello_world", "hello.go");
        Self::from(code)
    }

    /// A task for computing Fibonacci numbers in Go.
    pub fn example_go_task_fib() -> Self {
        let code = example_file_content!("go", "task_fib", "fib.go");
        Self::from(code)
    }

    /// Test code for the Fibonacci task in Go.
    pub fn example_go_task_fib_test() -> Self {
        let code = example_file_content!("go", "task_fib", "fib_test.go");
        Self::from(code)
    }
}
