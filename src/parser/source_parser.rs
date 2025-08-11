pub trait SourceParser {
    fn line_comment_prefix(&self) -> Option<String>;
    fn line_docstring_prefix(&self) -> Option<String>;
    fn block_comment_start(&self) -> Option<String>;
    fn block_comment_end(&self) -> Option<String>;
    fn block_docstring_start(&self) -> Option<String>;
    fn block_docstring_end(&self) -> Option<String>;
}
