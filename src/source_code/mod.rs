pub struct SourceCode {
    pub content: String,
}

impl From<String> for SourceCode {
    fn from(value: String) -> Self {
        Self::from(value.as_str())
    }
}

impl From<&String> for SourceCode {
    fn from(value: &String) -> Self {
        Self::from(value.as_str())
    }
}

impl From<&str> for SourceCode {
    fn from(value: &str) -> Self {
        Self {
            content: value.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str() {
        let code = SourceCode::from("fn main() {}");
        assert_eq!(code.content, "fn main() {}");
    }

    #[test]
    fn from_string() {
        let code = SourceCode::from("fn main() {}".to_string());
        assert_eq!(code.content, "fn main() {}");
    }

    #[test]
    fn from_string_ref() {
        let code = SourceCode::from(&"fn main() {}".to_string());
        assert_eq!(code.content, "fn main() {}");
    }
}

mod examples;
