use super::Example;

use crate::SourceCode;

#[derive(Debug)]
pub enum GoExample {
    DemoHello,
    TaskFib,
    TaskFibTest,
}

macro_rules! go_example_from_file {
    ($name:ident) => {{
        let code = include_str!(concat!("source_files/", stringify!($name), ".go.txt"));
        $crate::SourceCode::from(code)
    }};
}

impl Example for GoExample {
    fn to_source_code(&self) -> SourceCode {
        match self {
            GoExample::DemoHello => go_example_from_file!(demo_hello),
            GoExample::TaskFib => go_example_from_file!(task_fib),
            GoExample::TaskFibTest => go_example_from_file!(task_fib_test),
        }
    }

    fn file_name(&self) -> String {
        match self {
            GoExample::DemoHello => "demo_hello.go".to_string(),
            GoExample::TaskFib => "task_fib.go".to_string(),
            GoExample::TaskFibTest => "task_fib_test.go".to_string(),
        }
    }
}
