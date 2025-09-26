#![doc = include_str!("README.md")]

use crate::SourceCode;

use std::path::Path;

pub trait Example: Sized {
    /// Creates a `SourceCode` instance from this example.
    fn to_source_code(&self) -> SourceCode;

    /// Returns the file name to use when writing this example.
    fn file_name(&self) -> String;

    /// Returns a list of all examples defined in this impl of `Example`.
    fn all() -> Vec<Self>;

    /// Writes the example to a file at the given directory.
    /// Creates the directory if it doesn't exist.
    /// Creates the file if it doesn't exist or overwrites it if it does.
    fn write_file<P: AsRef<Path>>(&self, dir: P) -> std::io::Result<()> {
        let dir = dir.as_ref();
        if !dir.exists() {
            std::fs::create_dir_all(dir)?;
        }
        let source_code = self.to_source_code();
        let file_path = dir.join(self.file_name());
        source_code.write_file(&file_path)?;
        println!("Example written to {}", file_path.display());
        Ok(())
    }

    /// Writes all examples from this impl of `Example` to files at the given directory.
    /// Creates the directory if it doesn't exist.
    /// Creates each file if it doesn't exist or overwrites it if it does.
    fn write_all<P: AsRef<Path>>(dir: P) -> std::io::Result<()> {
        for example in Self::all() {
            example.write_file(&dir)?;
        }
        Ok(())
    }
}

mod go;
pub use go::GoExample;
