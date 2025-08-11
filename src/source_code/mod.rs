pub struct SourceCode {
    lines: Vec<String>,
}

impl SourceCode {
    /// Returns the content as a single string.
    pub fn content(&self) -> String {
        self.lines.join("\n")
    }
}

mod constructors;
mod examples;
