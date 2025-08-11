use crate::SourceParser;

pub struct SourceParserConfig {
    pub line_comment_prefix: Option<String>,
    pub line_docstring_prefix: Option<String>,
    pub block_comment_start: Option<String>,
    pub block_comment_end: Option<String>,
    pub block_docstring_start: Option<String>,
    pub block_docstring_end: Option<String>,
}

impl SourceParserConfig {
    pub fn go() -> Self {
        Self {
            line_comment_prefix: Some("//".to_string()),
            line_docstring_prefix: Some("//".to_string()),
            block_comment_start: Some("/*".to_string()),
            block_comment_end: Some("*/".to_string()),
            block_docstring_start: Some("/*".to_string()),
            block_docstring_end: Some("*/".to_string()),
        }
    }

    pub fn plain_text() -> Self {
        Self {
            line_comment_prefix: None,
            line_docstring_prefix: None,
            block_comment_start: None,
            block_comment_end: None,
            block_docstring_start: None,
            block_docstring_end: None,
        }
    }
}

impl SourceParser for SourceParserConfig {
    fn line_comment_prefix(&self) -> Option<String> {
        self.line_comment_prefix.clone()
    }

    fn line_docstring_prefix(&self) -> Option<String> {
        self.line_docstring_prefix.clone()
    }

    fn block_comment_start(&self) -> Option<String> {
        self.block_comment_start.clone()
    }

    fn block_comment_end(&self) -> Option<String> {
        self.block_comment_end.clone()
    }

    fn block_docstring_start(&self) -> Option<String> {
        self.block_docstring_start.clone()
    }

    fn block_docstring_end(&self) -> Option<String> {
        self.block_docstring_end.clone()
    }
}
