use super::SourceCode;

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

impl GoExample {
    pub fn to_source_code(&self) -> SourceCode {
        match self {
            GoExample::DemoHello => go_example_from_file!(demo_hello),
            GoExample::TaskFib => go_example_from_file!(task_fib),
            GoExample::TaskFibTest => go_example_from_file!(task_fib_test),
        }
    }
}
