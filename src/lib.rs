mod parser;
mod source_code;

pub use parser::{SourceParser, SourceParserConfig};
pub use source_code::SourceCode;

// Re-export example enums.
// TODO: Is there a more elegant way to do this?
pub use source_code::examples::GoExample;
