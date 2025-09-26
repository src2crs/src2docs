#![doc = include_str!("README.md")]

use crate::SourceCode;

use std::path::Path;

pub trait Example {
    fn to_source_code(&self) -> SourceCode;

    fn file_name(&self) -> String;

    fn write_file<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
        let source_code = self.to_source_code();
        source_code.write_file(path)
    }

    fn write_all<P: AsRef<Path>>(path: P) -> std::io::Result<()> {
        for example in &[
            GoExample::DemoHello,
            GoExample::TaskFib,
            GoExample::TaskFibTest,
        ] {
            let file_path = path.as_ref().join(example.file_name());
            example.write_file(&file_path)?;
            println!("Example written to {}", file_path.display());
        }
        Ok(())
    }
}

mod go;
pub use go::GoExample;
