use super::SourceCode;
use std::path::Path;

impl SourceCode {
    /// Export the source code as a single string.
    pub fn to_string(&self) -> String {
        self.lines.join("\n")
    }

    /// Write the source code to a file at the given path.
    /// Creates the file if it doesn't exist, and overwrites it if it does.
    pub fn write_file<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
        std::fs::write(path, self.to_string())
    }
}
