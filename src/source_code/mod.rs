pub struct SourceCode {
    pub content: String,
}

impl SourceCode {
    pub fn new(content: String) -> Self {
        SourceCode { content }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let code = SourceCode::new("fn main() {}".into());
        assert_eq!(code.content, "fn main() {}");
    }
}
